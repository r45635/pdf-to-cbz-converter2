import { useState, useCallback } from 'react';
import { useTranslation } from '@/lib/useTranslation';
import LanguageSelector from '@/components/LanguageSelector';
import TauriBatchUploader from '@/components/TauriBatchUploader';
import BatchSettings from '@/components/BatchSettings';
import BatchResults from '@/components/BatchResults';
import * as TauriClient from '@/lib/tauri-client';
import { BatchFileState } from '@/lib/batch-types';

type ConversionMode = 'pdf-to-cbz' | 'cbz-to-pdf';

interface BatchSettings {
  dpi: number;
  format: TauriClient.ImageFormat;
  quality: number;
  expirationMinutes: number;
  directExtract: boolean;
}

interface BatchProps {
  onNavigateToHome?: () => void;
}

export default function Batch({ onNavigateToHome }: BatchProps) {
  const { lang, setLang, t } = useTranslation();
  const [mode, setMode] = useState<ConversionMode>('pdf-to-cbz');
  const [files, setFiles] = useState<BatchFileState[]>([]);
  const [isConverting, setIsConverting] = useState(false);
  const [settings, setSettings] = useState<BatchSettings>({
    dpi: 0, // 0 = Auto-detect native DPI (True Lossless mode)
    format: 'png', // PNG for true lossless (0% compression)
    quality: 98,  // Max quality for JPEG fallback
    expirationMinutes: 60,
    directExtract: false, // Direct extraction disabled by default - only works for PDFs with embedded JPEG images
  });
  const [globalProgress, setGlobalProgress] = useState({
    completedFiles: 0,
    totalFiles: 0,
    currentFileProgress: 0,
  });

  // File addition handler from drag & drop or file dialog
  const handleAddFiles = useCallback((newFiles: BatchFileState[]) => {
    setFiles(prev => [...prev, ...newFiles]);
  }, []);

  // Update file status
  const updateFileStatus = useCallback((
    fileId: string,
    status: BatchFileState['status'],
    error?: string,
    result?: BatchFileState['result'],
    progress?: number,
    currentPage?: number,
    totalPages?: number
  ) => {
    setFiles(prev => prev.map(f =>
      f.id === fileId
        ? { ...f, status, error, result, progress, currentPage, totalPages }
        : f
    ));
  }, []);

  // Remove file from list
  const removeFile = useCallback((fileId: string) => {
    setFiles(prev => prev.filter(f => f.id !== fileId));
  }, []);

  // Clear all files
  const clearFiles = useCallback(() => {
    if (!isConverting) {
      setFiles([]);
      setGlobalProgress({ completedFiles: 0, totalFiles: 0, currentFileProgress: 0 });
    }
  }, [isConverting]);

  // Batch conversion handler
  const handleConvert = useCallback(async () => {
    if (files.length === 0) return;

    setIsConverting(true);
    setGlobalProgress({ completedFiles: 0, totalFiles: files.length, currentFileProgress: 0 });

    for (let i = 0; i < files.length; i++) {
      const file = files[i];
      const filePath = file.filePath;

      if (!filePath) {
        updateFileStatus(file.id, 'error', 'File path not available');
        continue;
      }

      try {
        updateFileStatus(file.id, 'analyzing', undefined, undefined, 0);

        let imageData: Uint8Array;

        if (mode === 'pdf-to-cbz') {
          // Get DPI for this file
          let fileDpi = settings.dpi as number;

          // If auto DPI (0), analyze the file to get native DPI
          if (fileDpi === 0) {
            try {
              const analysis = await TauriClient.analyzePdf(filePath);
              fileDpi = analysis.nativeDpi;
            } catch (err) {
              // If analysis fails, use default 150 DPI
              fileDpi = 150;
            }
          }

          // Convert the file with progress tracking
          imageData = await TauriClient.convertPdfToCbz(
            filePath,
            fileDpi,
            settings.format,
            settings.quality,
            (progress) => {
              // Update the current file's progress
              updateFileStatus(
                file.id,
                'converting',
                undefined,
                undefined,
                progress.percentage,
                progress.currentPage,
                progress.totalPages
              );
              // Update global progress
              setGlobalProgress({
                completedFiles: i,
                totalFiles: files.length,
                currentFileProgress: progress.percentage,
              });
            },
            settings.directExtract
          );
        } else {
          // TODO: Implement CBZ to PDF conversion
          throw new Error('CBZ to PDF conversion not yet implemented');
        }

        // Save the file automatically
        const outputPath = filePath.replace(/\.(pdf|cbz)$/i, mode === 'pdf-to-cbz' ? '.cbz' : '.pdf');
        await TauriClient.saveDataToFile(imageData, outputPath);

        // Mark as completed with 100% progress
        updateFileStatus(file.id, 'done', undefined, {
          outputName: outputPath.split(/[/\\]/).pop() || outputPath,
          sizeMB: imageData.length / (1024 * 1024),
          pageCount: 0, // TODO: Get actual page count
        }, 100);

      } catch (err) {
        const errorMessage = err instanceof Error ? err.message : 'Conversion failed';
        updateFileStatus(file.id, 'error', errorMessage);
      }

      // Update global progress
      setGlobalProgress(prev => ({
        ...prev,
        completedFiles: i + 1,
        currentFileProgress: 0,
      }));
    }

    setIsConverting(false);
  }, [files, mode, settings, updateFileStatus]);

  // Cancel conversion
  const handleCancel = useCallback(() => {
    setIsConverting(false);
    // Reset processing files
    setFiles(prev => prev.map(f =>
      f.status === 'analyzing' || f.status === 'converting'
        ? { ...f, status: 'pending' as const, progress: 0, currentPage: undefined, totalPages: undefined }
        : f
    ));
    setGlobalProgress({ completedFiles: 0, totalFiles: 0, currentFileProgress: 0 });
  }, []);

  // Calculate stats
  const stats = {
    total: files.length,
    completed: files.filter(f => f.status === 'done').length,
    errors: files.filter(f => f.status === 'error').length,
    pending: files.filter(f => f.status === 'pending').length,
    processing: files.filter(f => f.status === 'analyzing' || f.status === 'converting').length,
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 dark:from-gray-900 dark:to-gray-800">
      {/* Header */}
      <header className="bg-white dark:bg-gray-800 shadow-sm">
        <div className="max-w-7xl mx-auto px-4 py-2 sm:px-6 lg:px-8">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-4">
              <button
                onClick={onNavigateToHome}
                className="text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white transition-colors"
              >
                <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" />
                </svg>
              </button>
              <h1 className="text-2xl font-bold text-gray-900 dark:text-white">
                {t('batchConversion')}
              </h1>
            </div>
            <div className="flex items-center gap-4">
              <LanguageSelector lang={lang} setLang={setLang} />
              <button
                onClick={onNavigateToHome}
                className="px-4 py-2 text-gray-700 dark:text-gray-300 hover:text-gray-900 dark:hover:text-white transition-colors"
              >
                {t('singleFileMode')}
              </button>
            </div>
          </div>
        </div>
      </header>

      <main className="max-w-7xl mx-auto px-4 py-4 sm:px-6 lg:px-8">
        {/* Mode Selector */}
        <div className="mb-3 flex gap-2 bg-white dark:bg-gray-800 p-1 rounded-lg shadow">
          <button
            onClick={() => setMode('pdf-to-cbz')}
            disabled={isConverting}
            className={`flex-1 px-4 py-2 rounded-lg font-medium transition-colors disabled:opacity-50 ${
              mode === 'pdf-to-cbz'
                ? 'bg-indigo-600 text-white'
                : 'bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-200'
            }`}
          >
            {t('pdfToCbz')}
          </button>
          <button
            onClick={() => setMode('cbz-to-pdf')}
            disabled={isConverting}
            className={`flex-1 px-4 py-2 rounded-lg font-medium transition-colors disabled:opacity-50 ${
              mode === 'cbz-to-pdf'
                ? 'bg-indigo-600 text-white'
                : 'bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-200'
            }`}
          >
            {t('cbzToPdf')}
          </button>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-3 gap-3">
          {/* Left Column: File Upload & Settings */}
          <div className="lg:col-span-1 space-y-3">
            {/* File Upload */}
            <div className="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-4">
              <h2 className="text-base font-semibold mb-2 text-gray-900 dark:text-white">
                {t('files')}
              </h2>

              <TauriBatchUploader
                files={files}
                onFilesAdd={handleAddFiles}
                onFileRemove={removeFile}
                onClearAll={clearFiles}
                disabled={isConverting}
                mode={mode}
                maxFiles={10}
              />
            </div>

            {/* Settings */}
            <BatchSettings
              mode={mode}
              settings={settings}
              onSettingsChange={setSettings}
              disabled={isConverting}
            />

            {/* Action Buttons */}
            <div className="space-y-2">
              {!isConverting ? (
                <button
                  onClick={handleConvert}
                  disabled={files.length === 0}
                  className="w-full px-4 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 disabled:bg-gray-400 disabled:cursor-not-allowed font-medium transition-colors"
                >
                  {t('startConversion')}
                </button>
              ) : (
                <button
                  onClick={handleCancel}
                  className="w-full px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 font-medium transition-colors"
                >
                  {t('cancel')}
                </button>
              )}
            </div>

            {/* Stats */}
            {files.length > 0 && (
              <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-4">
                <h3 className="text-sm font-semibold mb-3 text-gray-900 dark:text-white">
                  Status
                </h3>
                <div className="space-y-2 text-sm">
                  <div className="flex justify-between">
                    <span className="text-gray-600 dark:text-gray-400">Total:</span>
                    <span className="font-medium text-gray-900 dark:text-white">{stats.total}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600 dark:text-gray-400">Completed:</span>
                    <span className="font-medium text-green-600 dark:text-green-400">{stats.completed}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600 dark:text-gray-400">Processing:</span>
                    <span className="font-medium text-blue-600 dark:text-blue-400">{stats.processing}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600 dark:text-gray-400">Pending:</span>
                    <span className="font-medium text-gray-600 dark:text-gray-400">{stats.pending}</span>
                  </div>
                  {stats.errors > 0 && (
                    <div className="flex justify-between">
                      <span className="text-gray-600 dark:text-gray-400">Errors:</span>
                      <span className="font-medium text-red-600 dark:text-red-400">{stats.errors}</span>
                    </div>
                  )}
                </div>
              </div>
            )}
          </div>

          {/* Right Column: Results */}
          <div className="lg:col-span-2">
            {/* Global Progress */}
            {isConverting && (
              <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-4 mb-3">
                <h3 className="text-base font-semibold mb-2 text-gray-900 dark:text-white">
                  {t('globalProgress')}
                </h3>
                <div className="space-y-2">
                  <div className="flex justify-between text-sm">
                    <span className="text-gray-600 dark:text-gray-400">
                      {t('conversionInProgress')}
                    </span>
                    <span className="font-medium text-gray-900 dark:text-white">
                      {globalProgress.completedFiles} / {globalProgress.totalFiles}
                    </span>
                  </div>
                  <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
                    <div
                      className="bg-indigo-600 h-3 rounded-full transition-all duration-300"
                      style={{
                        width: `${((globalProgress.completedFiles + (globalProgress.currentFileProgress / 100)) / globalProgress.totalFiles) * 100}%`
                      }}
                    />
                  </div>
                  {globalProgress.currentFileProgress > 0 && (
                    <div className="text-sm text-gray-600 dark:text-gray-400">
                      Current file: {Math.round(globalProgress.currentFileProgress)}%
                    </div>
                  )}
                </div>
              </div>
            )}

            {/* Results */}
            <BatchResults
              files={files}
              onRemove={removeFile}
              disabled={isConverting}
            />

            {files.length === 0 && (
              <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-12 text-center">
                <svg
                  className="w-16 h-16 text-gray-400 mx-auto mb-4"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
                  />
                </svg>
                <p className="text-gray-500 dark:text-gray-400">
                  {mode === 'pdf-to-cbz' ? t('batchDescPdf') : t('batchDescCbz')}
                </p>
              </div>
            )}
          </div>
        </div>
      </main>

      {/* Footer */}
      <footer className="mt-12 py-6 text-center text-gray-600 dark:text-gray-400 text-sm">
        <p>
          {t('footer')} • {t('madeWith')} ❤️
        </p>
      </footer>
    </div>
  );
}
