const { invoke } = window.__TAURI__.core;

let currentMode = 'pdf-to-cbz';
let selectedFile = null;

// Initialize
document.addEventListener('DOMContentLoaded', () => {
    setupEventListeners();
    setupDragDrop();
});

function setupEventListeners() {
    // Mode selector
    document.getElementById('pdfMode').addEventListener('click', () => setMode('pdf-to-cbz'));
    document.getElementById('cbzMode').addEventListener('click', () => setMode('cbz-to-pdf'));

    // File selection
    document.getElementById('selectBtn').addEventListener('click', () => {
        document.getElementById('fileInput').click();
    });

    document.getElementById('fileInput').addEventListener('change', (e) => {
        if (e.target.files.length > 0) {
            selectFile(e.target.files[0]);
        }
    });

    // Clear button
    document.getElementById('clearBtn').addEventListener('click', clearFile);

    // DPI selector
    document.getElementById('dpiSelect').addEventListener('change', (e) => {
        const customDpi = document.getElementById('customDpi');
        if (e.target.value === 'custom') {
            customDpi.style.display = 'block';
            customDpi.focus();
        } else {
            customDpi.style.display = 'none';
        }
    });

    // Convert button
    document.getElementById('convertBtn').addEventListener('click', convert);
}

function setupDragDrop() {
    const dropZone = document.getElementById('dropZone');

    ['dragenter', 'dragover', 'dragleave', 'drop'].forEach(eventName => {
        dropZone.addEventListener(eventName, preventDefaults, false);
        document.body.addEventListener(eventName, preventDefaults, false);
    });

    ['dragenter', 'dragover'].forEach(eventName => {
        dropZone.addEventListener(eventName, highlight, false);
    });

    ['dragleave', 'drop'].forEach(eventName => {
        dropZone.addEventListener(eventName, unhighlight, false);
    });

    dropZone.addEventListener('drop', handleDrop, false);
}

function preventDefaults(e) {
    e.preventDefault();
    e.stopPropagation();
}

function highlight(e) {
    document.getElementById('dropZone').classList.add('dragover');
}

function unhighlight(e) {
    document.getElementById('dropZone').classList.remove('dragover');
}

function handleDrop(e) {
    const files = e.dataTransfer.files;
    if (files.length > 0) {
        selectFile(files[0]);
    }
}

function setMode(mode) {
    currentMode = mode;

    // Update button states
    document.getElementById('pdfMode').classList.toggle('active', mode === 'pdf-to-cbz');
    document.getElementById('cbzMode').classList.toggle('active', mode === 'cbz-to-pdf');

    // Show/hide DPI options
    const pdfOptions = document.getElementById('pdfOptions');
    if (mode === 'pdf-to-cbz') {
        pdfOptions.style.display = 'block';
    } else {
        pdfOptions.style.display = 'none';
    }

    clearFile();
}

function selectFile(file) {
    // Validate file type
    const ext = file.name.split('.').pop().toLowerCase();
    const validExts = currentMode === 'pdf-to-cbz' ? ['pdf'] : ['cbz', 'cbr'];

    if (!validExts.includes(ext)) {
        showStatus(`Invalid file type. Expected ${validExts.join(', ')} but got ${ext}`, 'error');
        return;
    }

    selectedFile = file;

    // Update UI
    document.getElementById('fileInfo').style.display = 'flex';
    document.getElementById('dropZone').style.display = 'none';
    document.getElementById('fileName').textContent = file.name;
    document.getElementById('fileSize').textContent = formatFileSize(file.size);
    document.getElementById('convertBtn').disabled = false;

    clearStatus();
}

function clearFile() {
    selectedFile = null;
    document.getElementById('fileInfo').style.display = 'none';
    document.getElementById('dropZone').style.display = 'block';
    document.getElementById('convertBtn').disabled = true;
    document.getElementById('fileInput').value = '';
    clearStatus();
}

async function convert() {
    if (!selectedFile) {
        showStatus('No file selected', 'error');
        return;
    }

    try {
        showProgress(true);

        // Get output path
        const ext = currentMode === 'pdf-to-cbz' ? 'cbz' : 'pdf';
        const outputName = selectedFile.name.replace(/\.[^.]+$/, `.${ext}`);
        const outputPath = await invoke('get_save_path', { filename: outputName });

        // Get DPI if PDF mode
        let dpi = 300;
        if (currentMode === 'pdf-to-cbz') {
            const dpiSelect = document.getElementById('dpiSelect');
            if (dpiSelect.value === 'custom') {
                const customDpi = parseInt(document.getElementById('customDpi').value);
                if (isNaN(customDpi) || customDpi < 72 || customDpi > 1200) {
                    throw new Error('Invalid DPI value. Please enter a number between 72 and 1200');
                }
                dpi = customDpi;
            } else {
                dpi = parseInt(dpiSelect.value);
            }
        }

        // Call the appropriate converter
        updateProgress(0, 'Initializing conversion...');

        let result;
        if (currentMode === 'pdf-to-cbz') {
            result = await invoke('convert_pdf_to_cbz', {
                inputPath: selectedFile.path,
                outputPath: outputPath,
                dpi: dpi,
            });
        } else {
            result = await invoke('convert_cbz_to_pdf', {
                inputPath: selectedFile.path,
                outputPath: outputPath,
            });
        }

        // Simulate progress completion
        updateProgress(100, 'Conversion complete!');
        showStatus(`✓ ${result}`, 'success');
        clearFile();

    } catch (error) {
        showStatus(`✗ Error: ${error}`, 'error');
    } finally {
        showProgress(false);
    }
}

function formatFileSize(bytes) {
    if (bytes === 0) return '0 Bytes';
    const k = 1024;
    const sizes = ['Bytes', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + ' ' + sizes[i];
}

function showProgress(show) {
    document.getElementById('progressSection').style.display = show ? 'block' : 'none';
}

function updateProgress(percent, text) {
    document.getElementById('progressFill').style.width = percent + '%';
    document.getElementById('progressText').textContent = text;
}

function showStatus(message, type) {
    const statusDiv = document.getElementById('statusMessage');
    statusDiv.textContent = message;
    statusDiv.className = `status-message ${type}`;
    statusDiv.style.display = 'block';
}

function clearStatus() {
    document.getElementById('statusMessage').style.display = 'none';
}
