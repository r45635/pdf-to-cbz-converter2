import { useState, useCallback, useEffect, useRef } from 'react';
import { useTranslation } from '@/lib/useTranslation';
import LanguageSelector from '@/components/LanguageSelector';
import * as TauriClient from '@/lib/tauri-client';
import { listen } from '@tauri-apps/api/event';

type ConversionMode = 'pdf-to-cbz' | 'cbz-to-pdf';

interface BatchFile {
  path: string;
  name: string;
  savePath?: string;
  sourceSize?: number;  // File size in bytes
  convertedSize?: number;  // Converted file size in bytes
  status: 'pending' | 'converting' | 'completed' | 'error' | 'cancelled';
  progress: number;
  error?: string;
}

interface HomeProps {}

export default function Home({}: HomeProps) {
  const { lang, setLang, t } = useTranslation();
  const [mode, setMode] = useState<ConversionMode>('pdf-to-cbz');

  // Build timestamp - set once on mount (removed verbose logging for production)
  const [buildTime] = useState(() => {
    const now = new Date();
    return now.toLocaleTimeString('en-US', {
      hour12: true,
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit'
    });
  });

  // Batch mode (works for single or multiple files)
  const [batchFiles, setBatchFiles] = useState<BatchFile[]>([]);
  const [isDragging, setIsDragging] = useState(false);
  
  // Note: Progress listener is handled per-conversion in tauri-client.ts
  // No global listener needed - it was causing the infinite restart bug!

  // Options
  const [dpi, setDpi] = useState<string>('200'); // 200 DPI - Default recommended
  const [quality, setQuality] = useState(85);  // Quality 85 - Balanced
  const [lossless, setLossless] = useState(false);  // Lossless mode disabled by default

  // Status
  const [error, setError] = useState<string | null>(null);
  const [isCancelling, setIsCancelling] = useState(false);
  const [isProcessing, setIsProcessing] = useState(false);  // Prevent multiple simultaneous conversions
  const cancelledRef = useRef(false);
  const conversionIdRef = useRef(0);  // Track conversion ID to detect unwanted restarts

  // Log isProcessing changes
  useEffect(() => {
    console.log(`[STATE] isProcessing changed to: ${isProcessing}`);
  }, [isProcessing]);

  // Get effective DPI
  const effectiveDpi = parseInt(dpi, 10) || 150;

  // File selection handler - always adds to batch
  const handleFileSelect = useCallback(async (multiple = true) => {
    console.log('[DEBUG] handleFileSelect called, mode:', mode, 'multiple:', multiple);
    setError(null);
    
    try {
      const result = mode === 'pdf-to-cbz' 
        ? await TauriClient.selectPdfFile(multiple)
        : await TauriClient.selectCbzFile(multiple);
      
      console.log('[DEBUG] Selected file(s):', result);
      if (!result) {
        console.log('[DEBUG] No file selected');
        return;
      }

      // Handle single or multiple files - always add to batch
      const paths = Array.isArray(result) ? result : [result];
      const newFiles = await Promise.all(paths.map(async (path) => {
        const size = await TauriClient.getFileSize(path);
        // Extract filename using the appropriate separator
        const separator = path.includes('\\') ? '\\' : '/';
        const fileName = path.split(separator).pop() || 'file';
        return {
          path,
          name: fileName,
          sourceSize: size,
          status: 'pending' as const,
          progress: 0,
        };
      }));

      // Add to existing files (avoid duplicates)
      setBatchFiles(prev => {
        const existingPaths = new Set(prev.map(f => f.path));
        const filesToAdd = newFiles.filter(f => !existingPaths.has(f.path));
        return [...prev, ...filesToAdd];
      });
    } catch (err) {
      console.error('[ERROR] File selection failed:', err);
      setError(err instanceof Error ? err.message : 'File selection failed');
    }
  }, [mode]);

  // Batch conversion handler
  const handleBatchConvert = useCallback(async () => {
    // Increment conversion ID and capture it for this run
    conversionIdRef.current += 1;
    const thisConversionId = conversionIdRef.current;

    console.log(`\n${'='.repeat(80)}`);
    console.log(`[CONV #${thisConversionId}] handleBatchConvert CALLED`);
    console.log(`[CONV #${thisConversionId}] Stack trace:`, new Error().stack?.split('\n').slice(1, 4).join('\n'));
    console.log(`[CONV #${thisConversionId}] batchFiles: ${batchFiles.length}, isProcessing: ${isProcessing}`);
    console.log(`[CONV #${thisConversionId}] statuses: [${batchFiles.map(f => f.status).join(',')}]`);
    console.log('='.repeat(80));

    if (batchFiles.length === 0) {
      console.log(`[CONV #${thisConversionId}] EXIT - No files to process`);
      return;
    }

    // Prevent multiple simultaneous conversions
    if (isProcessing) {
      console.log(`[CONV #${thisConversionId}] EXIT - Conversion already in progress (blocked by isProcessing flag)`);
      return;
    }

    setIsProcessing(true);
    console.log(`[CONV #${thisConversionId}] Starting batch conversion for ${batchFiles.length} files`);

    // Reset cancellation flag
    setIsCancelling(false);
    cancelledRef.current = false;

    // Ask user to select destination directory
    const firstFilePath = batchFiles[0].path;
    // Use the platform-appropriate separator (backslash for Windows, slash for Unix)
    const separator = firstFilePath.includes('\\') ? '\\' : '/';
    const lastSeparatorIndex = firstFilePath.lastIndexOf(separator);
    const firstFileDir = firstFilePath.substring(0, lastSeparatorIndex);
    
    // Let user select destination directory (default to source folder)
    console.log('[DEBUG] Opening folder picker, default:', firstFileDir);
    const selectedDir = await TauriClient.selectDirectory(firstFileDir);
    
    if (!selectedDir) {
      console.log('[DEBUG] User cancelled directory selection');
      setIsProcessing(false);  // Reset flag when user cancels
      return; // User cancelled
    }
    
    const destinationDir = selectedDir;
    console.log('[DEBUG] User selected destination directory:', destinationDir);

    try {
      // Create a local copy of files to process
      const filesToProcess = [...batchFiles];

      // Reset all files to pending status
      setBatchFiles(filesToProcess.map(f => ({ ...f, status: 'pending', progress: 0, error: undefined, savePath: undefined })));

      // Process each file
      for (let i = 0; i < filesToProcess.length; i++) {
      // Check if conversion was cancelled
      if (cancelledRef.current) {
        console.log('[DEBUG] Batch conversion cancelled by user');
        // Mark remaining files as cancelled
        setBatchFiles(prev => prev.map((f) => 
          f.status === 'pending' ? { ...f, status: 'cancelled' } : f
        ));
        break;
      }

      const file = filesToProcess[i];
      console.log(`[DEBUG] Processing file ${i + 1}/${filesToProcess.length}: ${file.name}`);

      try {
        // Mark as converting
        console.log(`[FRONTEND] Starting conversion for file: ${file.path}`);
        console.log(`[FRONTEND] Mode: ${mode}, DPI: ${effectiveDpi}, Quality: ${quality}, Lossless: ${lossless}`);
        
        setBatchFiles(prev => prev.map((f) => 
          f.path === file.path ? { ...f, status: 'converting', progress: 0 } : f
        ));

        // Determine output file name and path
        console.log(`[DEBUG] file.name: "${file.name}"`);
        console.log(`[DEBUG] file.path: "${file.path}"`);
        console.log(`[DEBUG] destinationDir: "${destinationDir}"`);
        
        const defaultName = mode === 'pdf-to-cbz'
          ? file.name.replace(/\.pdf$/i, '.cbz')
          : file.name.replace(/\.(cbz|cbr)$/i, '.pdf');
        // Use the platform-appropriate separator
        const separator = destinationDir.includes('\\') ? '\\' : '/';
        const savePath = `${destinationDir}${separator}${defaultName}`;
        
        console.log(`[DEBUG] defaultName: "${defaultName}"`);
        console.log(`[DEBUG] savePath: "${savePath}"`);

        // Store the save path early
        setBatchFiles(prev => prev.map((f) =>
          f.path === file.path ? { ...f, savePath, progress: 5 } : f
        ));

        // Convert with progress callback
        console.log(`[FRONTEND] Calling conversion function...`);
        console.log(`[TIMING] Starting conversion at ${new Date().toLocaleTimeString()}`);

        let convertedSize: number;

        if (mode === 'pdf-to-cbz') {
          // Use DIRECT disk write (no IPC bottleneck for large files!)
          convertedSize = await TauriClient.convertPdfToCbzDirect(
            file.path,
            savePath,  // Write directly to disk
            effectiveDpi,
            quality,
            (progress) => {
              console.log(`[FRONTEND] Progress: ${progress.percentage}%`);
              setBatchFiles(prev => prev.map((f) =>
                f.path === file.path ? { ...f, progress: progress.percentage } : f
              ));
            },
            lossless
          );
        } else {
          // CBZ to PDF still uses old method (typically smaller files)
          const outputData = await TauriClient.convertCbzToPdf(
            file.path,
            (progress) => {
              console.log(`[FRONTEND] Progress: ${progress.percentage}%`);
              setBatchFiles(prev => prev.map((f) =>
                f.path === file.path ? { ...f, progress: progress.percentage } : f
              ));
            },
            lossless,
            quality
          );
          await TauriClient.saveDataToFile(outputData, savePath);
          convertedSize = outputData.length;
        }

        console.log(`[TIMING] Conversion completed at ${new Date().toLocaleTimeString()}`);
        console.log(`[FRONTEND] Output size: ${convertedSize} bytes (${(convertedSize / 1024 / 1024).toFixed(1)} MB)`);

        // Mark as completed (and clear any previous error messages)
        console.log(`[TIMING] Marking as completed at ${new Date().toLocaleTimeString()}`);
        setBatchFiles(prev => prev.map((f) =>
          f.path === file.path ? { ...f, status: 'completed', progress: 100, convertedSize, error: undefined } : f
        ));

        console.log(`[DEBUG] Successfully converted: ${file.name} -> ${savePath}`);

      } catch (err) {
        console.error(`[FRONTEND ERROR] ==================== CONVERSION FAILED ====================`);
        console.error(`[FRONTEND ERROR] File: ${file.name}`);
        console.error(`[FRONTEND ERROR] Error:`, err);
        console.error(`[FRONTEND ERROR] Error type:`, typeof err);
        console.error(`[FRONTEND ERROR] Error stack:`, err instanceof Error ? err.stack : 'N/A');
        console.error(`[FRONTEND ERROR] ============================================================`);
        
        // Check if it's a cancellation error
        const errorMessage = err instanceof Error ? err.message : 'Conversion failed';
        const isCancellation = errorMessage.includes('cancelled') || errorMessage.includes('Conversion cancelled');
        
        setBatchFiles(prev => prev.map((f) => 
          f.path === file.path ? { 
            ...f, 
            status: isCancellation ? 'cancelled' : 'error', 
            progress: 0, 
            error: isCancellation ? undefined : errorMessage 
          } : f
        ));
        
        // If cancelled, stop processing remaining files
        if (isCancellation) {
          console.log('[DEBUG] Conversion was cancelled, stopping batch');
          cancelledRef.current = true;
          setBatchFiles(prev => prev.map((f) => 
            f.status === 'pending' ? { ...f, status: 'cancelled' } : f
          ));
          break;
        }
      }
    }

      console.log(`[CONV #${thisConversionId}] FINISH - Batch conversion completed - all files done`);
      setIsCancelling(false);
      cancelledRef.current = false;
    } finally {
      // Always clear the processing flag
      console.log(`[CONV #${thisConversionId}] FINALLY - Clearing isProcessing flag`);
      setIsProcessing(false);
      console.log(`[CONV #${thisConversionId}] ${'='.repeat(60)}`);
      console.log(`[CONV #${thisConversionId}] CONVERSION FULLY COMPLETE`);
      console.log(`[CONV #${thisConversionId}] ${'='.repeat(60)}\n`);
    }
  }, [batchFiles, mode, effectiveDpi, quality, lossless]);

  // Cancel batch conversion
  const handleCancelBatch = useCallback(() => {
    console.log('[DEBUG] User requested to cancel batch conversion');
    setIsCancelling(true);
    cancelledRef.current = true;
    
    // Also call Rust cancellation to stop ongoing conversions
    TauriClient.cancelConversion().catch(err => {
      console.error('[ERROR] Failed to cancel conversion:', err);
    });
  }, []);

  // Restart batch conversion
  const handleRestartBatch = useCallback(() => {
    setBatchFiles(prev => prev.map(f => ({ ...f, status: 'pending', progress: 0, error: undefined })));
  }, []);

  // Clear and add new files
  const handleClearAndAddNew = useCallback(async () => {
    setBatchFiles([]);
    await handleFileSelect(true);
  }, [handleFileSelect]);

  // Handle file drop event from Tauri backend
  useEffect(() => {
    const unlisten = listen<string[]>('tauri://file-drop', async (event) => {
      console.log('[DEBUG] Tauri file drop event received:', event.payload);
      const paths = event.payload;

      if (paths.length === 0) {
        console.log('[DEBUG] No files in drop event');
        return;
      }

      setIsDragging(false);

      // Filter files based on mode
      const validFiles = paths.filter(f => {
        const lower = f.toLowerCase();
        if (mode === 'pdf-to-cbz') {
          return lower.endsWith('.pdf');
        } else {
          return lower.endsWith('.cbz') || lower.endsWith('.cbr');
        }
      });

      if (validFiles.length === 0) {
        setError(mode === 'pdf-to-cbz' ? 'Please drop PDF files only' : 'Please drop CBZ/CBR files only');
        return;
      }

      // Add all files to batch
      try {
        const newFiles = await Promise.all(validFiles.map(async (path) => {
          try {
            const size = await TauriClient.getFileSize(path);
            return {
              path,
              name: path.split('/').pop() || path.split('\\').pop() || 'file',
              sourceSize: size,
              status: 'pending' as const,
              progress: 0,
            };
          } catch (err) {
            console.error('[ERROR] Failed to get file size for:', path, err);
            return null;
          }
        }));

        const validNewFiles = newFiles.filter((f): f is BatchFile => f !== null);

        if (validNewFiles.length === 0) {
          setError('Unable to access dropped files. Please use "Add Files" button instead.');
          return;
        }

        // Add to existing files (avoid duplicates)
        setBatchFiles(prev => {
          const existingPaths = new Set(prev.map(f => f.path));
          const filesToAdd = validNewFiles.filter(f => !existingPaths.has(f.path));
          return [...prev, ...filesToAdd];
        });

        console.log('[DEBUG] Successfully added', validNewFiles.length, 'files from drop');
      } catch (err) {
        console.error('[ERROR] Failed to process dropped files:', err);
        setError('Failed to process dropped files');
      }
    });

    return () => {
      unlisten.then(fn => fn());
    };
  }, [mode]);

  // Handle visual feedback for drag over
  useEffect(() => {
    const handleDragOver = (e: DragEvent) => {
      e.preventDefault();
      e.stopPropagation();
      setIsDragging(true);
    };

    const handleDragLeave = (e: DragEvent) => {
      e.preventDefault();
      e.stopPropagation();
      // Only set to false if leaving the window entirely
      if (e.clientX === 0 && e.clientY === 0) {
        setIsDragging(false);
      }
    };

    document.addEventListener('dragover', handleDragOver);
    document.addEventListener('dragleave', handleDragLeave);

    return () => {
      document.removeEventListener('dragover', handleDragOver);
      document.removeEventListener('dragleave', handleDragLeave);
    };
  }, []);

  // Mode change handler
  const handleModeChange = useCallback((newMode: ConversionMode) => {
    setMode(newMode);
    setBatchFiles([]);
    setError(null);
  }, []);

  // Check if any conversion is in progress
  const isConverting = batchFiles.some(f => f.status === 'converting');
  const hasCompleted = batchFiles.some(f => f.status === 'completed');

  return (
    <div className="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 dark:from-gray-900 dark:to-gray-800">
      {/* Header */}
      <header className="bg-white dark:bg-gray-800 shadow-sm">
        <div className="max-w-7xl mx-auto px-4 py-3 sm:px-6 lg:px-8">
          <div className="flex items-center justify-between">
            <h1 className="text-2xl font-bold text-gray-900 dark:text-white">
              {t('title')}
            </h1>
            <div className="flex items-center gap-4">
              <LanguageSelector lang={lang} setLang={setLang} />
            </div>
          </div>
        </div>
      </header>

      <main className="max-w-7xl mx-auto px-4 py-4 sm:px-6 lg:px-8">
        {/* Mode Selector */}
        <div className="mb-4 flex gap-4 bg-white dark:bg-gray-800 p-2 rounded-lg shadow">
          <button
            onClick={() => handleModeChange('pdf-to-cbz')}
            className={`flex-1 px-4 py-2 rounded-lg font-medium transition-colors ${
              mode === 'pdf-to-cbz'
                ? 'bg-indigo-600 text-white'
                : 'bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-200'
            }`}
          >
            {t('pdfToCbz')}
          </button>
          <button
            onClick={() => handleModeChange('cbz-to-pdf')}
            className={`flex-1 px-4 py-2 rounded-lg font-medium transition-colors ${
              mode === 'cbz-to-pdf'
                ? 'bg-indigo-600 text-white'
                : 'bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-200'
            }`}
          >
            {t('cbzToPdf')}
          </button>
        </div>

        {/* File Upload Area */}
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-3 mb-4">
          <div
            className={`border-2 border-dashed rounded-lg p-4 text-center transition-colors ${
              isDragging
                ? 'border-indigo-500 bg-indigo-50 dark:bg-indigo-900/20'
                : 'border-gray-300 dark:border-gray-600 hover:border-indigo-500 hover:bg-indigo-50 dark:hover:bg-gray-700'
            }`}>
            <div className="flex flex-col items-center">
              <svg
                className="w-10 h-10 text-gray-400 mb-1"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"
                />
              </svg>
              <p className="text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                {mode === 'pdf-to-cbz' 
                  ? 'Drag & drop PDF file(s) here or click to select' 
                  : 'Drag & drop CBZ/CBR file(s) here or click to select'}
              </p>
              
              {/* Add Files Button */}
              <div className="mt-1">
                <button
                  onClick={() => handleFileSelect(true)}
                  className="px-4 py-1.5 bg-indigo-600 text-white text-sm rounded-lg hover:bg-indigo-700 transition-colors"
                >
                  📁 Add Files to List
                </button>
              </div>
              
              {batchFiles.length > 0 && (
                <p className="text-xs text-gray-500 dark:text-gray-400 mt-2">
                  {batchFiles.length} file(s) in list
                </p>
              )}
            </div>
          </div>

          {error && (
            <div className="mt-3 p-3 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg">
              <p className="text-red-700 dark:text-red-400">{error}</p>
            </div>
          )}
        </div>

        {/* Batch Mode Interface - Always visible when there are files */}
        {batchFiles.length > 0 && (
          <div className="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-4 mb-4">
            <div className="flex justify-between items-center mb-3">
              <h2 className="text-xl font-semibold text-gray-900 dark:text-white">
                Batch Conversion ({batchFiles.length} files)
              </h2>
              <div className="flex gap-2">
                <button
                  onClick={() => handleFileSelect(true)}
                  className="px-4 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors"
                >
                  ➕ Add More Files
                </button>
                <button
                  onClick={() => {
                    setBatchFiles([]);
                  }}
                  className="px-4 py-2 bg-gray-500 text-white rounded-lg hover:bg-gray-600 transition-colors"
                >
                  🗑️ Clear All
                </button>
              </div>
            </div>

            {/* Conversion Settings */}
            <div className="mb-3 p-3 bg-gray-50 dark:bg-gray-900 rounded-lg">
              <h3 className="text-sm font-semibold text-gray-900 dark:text-white mb-2">⚙️ Conversion Settings (Multi-Threading Enabled)</h3>
              <div className="grid grid-cols-1 md:grid-cols-3 gap-3">
                {mode === 'pdf-to-cbz' && !lossless && (
                  <div>
                    <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                      DPI Resolution
                    </label>
                    <select
                      value={dpi}
                      onChange={(e) => setDpi(e.target.value)}
                      className="w-full px-2 py-1 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white text-sm"
                    >
                      <option value="150">⚡ Fast (150 DPI - 20s for 270 pages)</option>
                      <option value="200">⭐ Balanced (200 DPI - 22s) - RECOMMENDED</option>
                      <option value="300">💎 High Quality (300 DPI - 27s)</option>
                    </select>
                    <p className="text-xs text-gray-500 dark:text-gray-400 mt-1">
                      {dpi === '150' && '📏 ~119MB for 270 pages - Mobile/Tablet optimized'}
                      {dpi === '200' && '📏 ~175MB for 270 pages - Best balance'}
                      {dpi === '300' && '📏 ~368MB for 270 pages - Desktop/Archive'}
                    </p>
                  </div>
                )}
                <div>
                  <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                    Mode
                  </label>
                  <div className="flex items-center gap-2">
                    <input
                      type="checkbox"
                      checked={lossless}
                      onChange={(e) => setLossless(e.target.checked)}
                      className="rounded border-gray-300 dark:border-gray-600"
                    />
                    <span className="text-sm text-gray-700 dark:text-gray-300">
                      Lossless Mode
                    </span>
                  </div>
                  <p className="text-xs text-gray-500 dark:text-gray-400 mt-1">
                    {lossless ? '⚠️ Slower but preserves original quality' : '⚡ Optimized with multi-threading'}
                  </p>
                </div>
                {!lossless && (
                  <div>
                    <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                      JPEG Quality ({quality})
                    </label>
                    <input
                      type="range"
                      min="50"
                      max="95"
                      value={quality}
                      onChange={(e) => setQuality(parseInt(e.target.value))}
                      className="w-full"
                    />
                    <div className="flex justify-between text-xs text-gray-500 dark:text-gray-400 mt-1">
                      <span>Small (50)</span>
                      <span>Best (95)</span>
                    </div>
                  </div>
                )}
              </div>
              
            </div>

            {/* Batch Files List */}
            <div className="overflow-x-auto">
              <table className="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
                <thead className="bg-gray-50 dark:bg-gray-900">
                  <tr>
                    <th className="px-3 py-2 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                      Source ⇒ Destination
                    </th>
                    <th className="px-3 py-2 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                      Status
                    </th>
                    <th className="px-3 py-2 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                      Progress
                    </th>
                  </tr>
                </thead>
                <tbody className="bg-white dark:bg-gray-800 divide-y divide-gray-200 dark:divide-gray-700">
                  {batchFiles.map((file, idx) => (
                    <tr key={idx}>
                      <td className="px-3 py-2 text-sm">
                        <div className="flex flex-col gap-1">
                          <div className="flex items-center gap-2">
                            <button
                              onClick={() => TauriClient.openFile(file.path)}
                              className="text-blue-600 dark:text-blue-400 hover:underline text-left break-all"
                              title={file.path}
                            >
                              {file.name}
                            </button>
                            {file.sourceSize && (
                              <span className="text-xs text-gray-500 dark:text-gray-400 whitespace-nowrap">
                                ({(file.sourceSize / 1024 / 1024).toFixed(1)} MB)
                              </span>
                            )}
                          </div>
                          {file.savePath && (
                            <>
                              <div className="text-gray-400 text-xs">↓ Converted to:</div>
                              <div className="flex items-start gap-2">
                                <div className="flex-1">
                                  <button
                                    onClick={() => TauriClient.openFile(file.savePath!)}
                                    className="text-green-600 dark:text-green-400 hover:underline text-left break-all"
                                    title={file.savePath}
                                  >
                                    {file.savePath.split('/').pop() || file.savePath.split('\\').pop()}
                                  </button>
                                  {file.convertedSize && (
                                    <span className="text-xs text-gray-500 dark:text-gray-400 ml-2">
                                      ({(file.convertedSize / 1024 / 1024).toFixed(1)} MB)
                                    </span>
                                  )}
                                </div>
                              </div>
                            </>
                          )}
                        </div>
                      </td>
                      <td className="px-3 py-2 whitespace-nowrap">
                        <span className={`px-2 inline-flex text-xs leading-5 font-semibold rounded-full ${
                          file.status === 'pending' ? 'bg-gray-100 text-gray-800' :
                          file.status === 'converting' ? 'bg-blue-100 text-blue-800' :
                          file.status === 'completed' ? 'bg-green-100 text-green-800' :
                          file.status === 'cancelled' ? 'bg-yellow-100 text-yellow-800' :
                          'bg-red-100 text-red-800'
                        }`}>
                          {file.status}
                        </span>
                        {file.error && (
                          <p className="text-xs text-red-600 mt-1">{file.error}</p>
                        )}
                      </td>
                      <td className="px-3 py-2 whitespace-nowrap">
                        <div className="flex items-center">
                          <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2 mr-2">
                            <div
                              className="bg-indigo-600 h-2 rounded-full transition-all duration-300"
                              style={{ width: `${file.progress}%` }}
                            />
                          </div>
                          <span className="text-sm text-gray-600 dark:text-gray-400">
                            {file.progress}%
                          </span>
                        </div>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>

            {/* Batch Control Buttons */}
            <div className="mt-3 flex gap-2">
              {!isConverting && !hasCompleted && (
                <button
                  onClick={handleBatchConvert}
                  disabled={batchFiles.length === 0 || isProcessing}
                  className="flex-1 px-4 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 disabled:bg-gray-400 disabled:cursor-not-allowed font-medium transition-colors"
                >
                  🚀 Start Batch Conversion
                </button>
              )}
              
              {isConverting && (
                <button
                  onClick={handleCancelBatch}
                  disabled={isCancelling}
                  className="flex-1 px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 disabled:bg-gray-400 disabled:cursor-not-allowed font-medium transition-colors"
                >
                  {isCancelling ? '⏹️ Cancelling...' : '⏹️ Stop Conversion'}
                </button>
              )}
              
              {hasCompleted && !isConverting && (
                <>
                  <button
                    onClick={handleRestartBatch}
                    disabled={isProcessing}
                    className="flex-1 px-4 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 disabled:bg-gray-400 disabled:cursor-not-allowed font-medium transition-colors"
                  >
                    🔄 Restart Batch Conversion
                  </button>
                  <button
                    onClick={handleClearAndAddNew}
                    className="flex-1 px-4 py-2 bg-purple-600 text-white rounded-lg hover:bg-purple-700 font-medium transition-colors"
                  >
                    🆕 Clear & Add New Files
                  </button>
                </>
              )}
              
              {isConverting && (
                <div className="flex-1 px-4 py-2 bg-blue-500 text-white rounded-lg font-medium text-center">
                  ⏳ Converting... ({batchFiles.filter(f => f.status === 'completed').length}/{batchFiles.length})
                </div>
              )}
            </div>
          </div>
        )}
      </main>

      {/* Footer */}
      <footer className="mt-8 py-4 text-center text-gray-600 dark:text-gray-400 text-sm">
        <p>
          {t('footer')} • {t('madeWith')} ❤️
        </p>
      </footer>
    </div>
  );
}
