// Audio Capture Demo - Main Entry Point
import { CaptureController } from './capture-controller';
import { CaptureGraphRenderer } from './capture-graph';
import { CSVExporter } from './csv-export';

// Initialize when DOM is ready
document.addEventListener('DOMContentLoaded', async () => {
  console.log('Audio Capture Demo initializing...');
  
  // Initialize controller
  const captureController = new CaptureController();
  let currentCaptureData: any = null;
  let graphRenderer: CaptureGraphRenderer | null = null;
  
  // Get UI Elements
  const elements = {
    inputDevice: document.getElementById('input-device') as HTMLSelectElement,
    outputDevice: document.getElementById('output-device') as HTMLSelectElement,
    signalType: document.getElementById('signal-type') as HTMLSelectElement,
    duration: document.getElementById('duration') as HTMLSelectElement,
    outputChannel: document.getElementById('output-channel') as HTMLSelectElement,
    sampleRate: document.getElementById('sample-rate') as HTMLSelectElement,
    inputVolume: document.getElementById('input-volume') as HTMLInputElement,
    outputVolume: document.getElementById('output-volume') as HTMLInputElement,
    inputVolumeValue: document.getElementById('input-volume-value') as HTMLElement,
    outputVolumeValue: document.getElementById('output-volume-value') as HTMLElement,
    startCapture: document.getElementById('start-capture') as HTMLButtonElement,
    stopCapture: document.getElementById('stop-capture') as HTMLButtonElement,
    exportCsv: document.getElementById('export-csv') as HTMLButtonElement,
    refreshDevices: document.getElementById('refresh-devices') as HTMLButtonElement,
    statusMessage: document.getElementById('status-message') as HTMLElement,
    captureProgress: document.getElementById('capture-progress') as HTMLElement,
    progressFill: document.getElementById('progress-fill') as HTMLElement,
    resultsContainer: document.getElementById('results-container') as HTMLElement,
    resultsInfo: document.getElementById('results-info') as HTMLElement,
    captureGraph: document.getElementById('capture-graph') as HTMLCanvasElement
  };
  
  // Volume slider handlers
  elements.inputVolume.addEventListener('input', (e) => {
    elements.inputVolumeValue.textContent = `${(e.target as HTMLInputElement).value}%`;
  });
  
  elements.outputVolume.addEventListener('input', (e) => {
    elements.outputVolumeValue.textContent = `${(e.target as HTMLInputElement).value}%`;
  });
  
  // Show status message
  function showStatus(message: string, type: 'info' | 'success' | 'error' = 'info') {
    elements.statusMessage.textContent = message;
    elements.statusMessage.className = `status-message show ${type}`;
    setTimeout(() => {
      elements.statusMessage.classList.remove('show');
    }, 5000);
  }
  
  // Load audio devices
  async function loadDevices() {
    try {
      showStatus('Loading audio devices...', 'info');
      const devices = await captureController.getAudioDevices();
      
      // Clear and populate input devices
      elements.inputDevice.innerHTML = '';
      devices.input.forEach(device => {
        const option = document.createElement('option');
        option.value = device.value;
        option.textContent = device.label;
        if (device.info) {
          option.title = device.info;
        }
        elements.inputDevice.appendChild(option);
      });
      
      // Clear and populate output devices
      elements.outputDevice.innerHTML = '';
      devices.output.forEach(device => {
        const option = document.createElement('option');
        option.value = device.value;
        option.textContent = device.label;
        if (device.info) {
          option.title = device.info;
        }
        elements.outputDevice.appendChild(option);
      });
      
      showStatus('Audio devices loaded successfully', 'success');
    } catch (error) {
      console.error('Failed to load devices:', error);
      showStatus(`Failed to load devices: ${(error as Error).message}`, 'error');
    }
  }
  
  // Start capture
  async function startCapture() {
    try {
      // Disable start button, enable stop button
      elements.startCapture.disabled = true;
      elements.stopCapture.disabled = false;
      elements.exportCsv.disabled = true;
      
      // Show progress
      elements.captureProgress.classList.add('show');
      elements.progressFill.style.width = '0%';
      elements.progressFill.textContent = '0%';
      
      // Hide previous results
      elements.resultsContainer.classList.remove('show');
      
      showStatus('Starting capture...', 'info');
      
      const params = {
        inputDevice: elements.inputDevice.value,
        outputDevice: elements.outputDevice.value,
        outputChannel: elements.outputChannel.value as 'left' | 'right' | 'both' | 'default',
        signalType: elements.signalType.value as 'sweep' | 'white' | 'pink',
        duration: parseInt(elements.duration.value),
        sampleRate: parseInt(elements.sampleRate.value),
        inputVolume: parseInt(elements.inputVolume.value),
        outputVolume: parseInt(elements.outputVolume.value)
      };
      
      console.log('Starting capture with params:', params);
      
      // Simulate progress
      const duration = params.duration * 1000;
      const startTime = Date.now();
      const progressInterval = setInterval(() => {
        const elapsed = Date.now() - startTime;
        const progress = Math.min(100, (elapsed / duration) * 100);
        elements.progressFill.style.width = `${progress}%`;
        elements.progressFill.textContent = `${Math.round(progress)}%`;
        
        if (progress >= 100) {
          clearInterval(progressInterval);
        }
      }, 100);
      
      const result = await captureController.startCapture(params);
      
      clearInterval(progressInterval);
      elements.progressFill.style.width = '100%';
      elements.progressFill.textContent = '100%';
      
      if (result.success) {
        showStatus('Capture completed successfully!', 'success');
        
        // Store the capture data
        currentCaptureData = {
          frequencies: result.frequencies,
          magnitudes: result.magnitudes,
          phases: result.phases,
          metadata: {
            timestamp: new Date(),
            deviceName: elements.inputDevice.options[elements.inputDevice.selectedIndex].text,
            signalType: params.signalType,
            duration: params.duration,
            sampleRate: params.sampleRate,
            outputChannel: params.outputChannel
          }
        };
        
        // Display results
        displayResults(currentCaptureData);
        
        // Enable export button
        elements.exportCsv.disabled = false;
      } else {
        throw new Error(result.error || 'Capture failed');
      }
    } catch (error) {
      console.error('Capture error:', error);
      showStatus(`Capture failed: ${(error as Error).message}`, 'error');
    } finally {
      // Reset buttons
      elements.startCapture.disabled = false;
      elements.stopCapture.disabled = true;
      
      // Hide progress after a moment
      setTimeout(() => {
        elements.captureProgress.classList.remove('show');
      }, 2000);
    }
  }
  
  // Stop capture
  function stopCapture() {
    captureController.stopCapture();
    showStatus('Capture stopped', 'info');
    
    // Reset buttons
    elements.startCapture.disabled = false;
    elements.stopCapture.disabled = true;
    elements.captureProgress.classList.remove('show');
  }
  
  // Display results
  function displayResults(data: any) {
    if (!data || !data.frequencies || !data.magnitudes) return;
    
    // Show results container
    elements.resultsContainer.classList.add('show');
    
    // Display info
    elements.resultsInfo.innerHTML = `
      <p><strong>Captured:</strong> ${data.frequencies.length} frequency points</p>
      <p><strong>Sample Rate:</strong> ${data.metadata.sampleRate} Hz</p>
      <p><strong>Signal Type:</strong> ${data.metadata.signalType}</p>
      <p><strong>Duration:</strong> ${data.metadata.duration} seconds</p>
    `;
    
    // Initialize graph renderer if needed
    if (!graphRenderer) {
      graphRenderer = new CaptureGraphRenderer(elements.captureGraph);
    }
    
    // Render the graph
    graphRenderer.renderGraph({
      frequencies: data.frequencies,
      rawMagnitudes: data.magnitudes,
      smoothedMagnitudes: data.magnitudes, // Use same data for now
      rawPhase: data.phases || [],
      smoothedPhase: data.phases || []
    });
  }
  
  // Export CSV
  function exportCsv() {
    if (!currentCaptureData) {
      showStatus('No capture data to export', 'error');
      return;
    }
    
    try {
      const exportData = {
        frequencies: currentCaptureData.frequencies,
        rawMagnitudes: currentCaptureData.magnitudes,
        smoothedMagnitudes: currentCaptureData.magnitudes, // Use same data for now
        rawPhase: currentCaptureData.phases || [],
        smoothedPhase: currentCaptureData.phases || [],
        metadata: currentCaptureData.metadata
      };
      
      CSVExporter.exportToCSV(exportData);
      showStatus('CSV exported successfully', 'success');
    } catch (error) {
      console.error('Export error:', error);
      showStatus(`Export failed: ${(error as Error).message}`, 'error');
    }
  }
  
  // Event handlers
  elements.startCapture.addEventListener('click', startCapture);
  elements.stopCapture.addEventListener('click', stopCapture);
  elements.exportCsv.addEventListener('click', exportCsv);
  elements.refreshDevices.addEventListener('click', loadDevices);
  
  // Initialize
  await loadDevices();
  
  console.log('Audio Capture Demo initialized');
});