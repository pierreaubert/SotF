// UI management and interaction functionality

import {
  OPTIMIZATION_DEFAULTS,
  OPTIMIZATION_LIMITS,
  OPTIMIZATION_STEPS,
} from "./optimization-constants";

export class UIManager {
  private form!: HTMLFormElement;
  private optimizeBtn!: HTMLButtonElement;
  private resetBtn!: HTMLButtonElement;
  private progressElement!: HTMLElement;
  private errorElement!: HTMLElement;

  // Modal elements
  private optimizationModal!: HTMLElement;
  private progressStatus!: HTMLElement;
  private elapsedTimeElement!: HTMLElement;
  private progressTableBody!: HTMLElement;
  private cancelOptimizationBtn!: HTMLButtonElement;
  private doneOptimizationBtn!: HTMLButtonElement;
  private modalCloseBtn!: HTMLButtonElement;
  private progressGraphElement!: HTMLElement;

  // Timing
  private optimizationStartTime: number = 0;

  // Audio testing elements
  private demoAudioSelect!: HTMLSelectElement;
  private eqOnBtn!: HTMLButtonElement;
  private eqOffBtn!: HTMLButtonElement;
  private listenBtn!: HTMLButtonElement;
  private stopBtn!: HTMLButtonElement;
  private audioStatus!: HTMLElement;
  private audioStatusText!: HTMLElement;
  private audioDuration!: HTMLElement;
  private audioPosition!: HTMLElement;
  private audioProgressFill!: HTMLElement;

  // Capture elements
  private captureBtn: HTMLButtonElement | null = null;
  private captureStatus: HTMLElement | null = null;
  private captureStatusText: HTMLElement | null = null;
  private captureProgressFill: HTMLElement | null = null;
  private captureWaveform: HTMLCanvasElement | null = null;
  private captureWaveformCtx: CanvasRenderingContext2D | null = null;
  private captureResult: HTMLElement | null = null;
  private captureClearBtn: HTMLButtonElement | null = null;
  private capturePlot: HTMLElement | null = null;
  private captureDeviceSelect: HTMLSelectElement | null = null;
  private sweepDurationSelect: HTMLSelectElement | null = null;
  private outputChannelSelect: HTMLSelectElement | null = null;
  private captureSampleRateSelect: HTMLSelectElement | null = null;
  private signalTypeSelect: HTMLSelectElement | null = null;

  // State
  private eqEnabled: boolean = true;
  private isResizing: boolean = false;
  private startX: number = 0;
  private startWidth: number = 0;

  // Capture modal elements
  private captureModal: HTMLElement | null = null;
  private captureModalGraph: HTMLCanvasElement | null = null;
  private captureModalPlaceholder: HTMLElement | null = null;
  private captureModalProgress: HTMLElement | null = null;
  private captureModalProgressFill: HTMLElement | null = null;
  private captureModalStatus: HTMLElement | null = null;
  private captureModalStart: HTMLButtonElement | null = null;
  private captureModalStop: HTMLButtonElement | null = null;
  private captureModalExport: HTMLButtonElement | null = null;
  private captureModalCancel: HTMLButtonElement | null = null;
  private captureModalClose: HTMLButtonElement | null = null;
  
  // Modal capture control elements
  private modalCaptureDevice: HTMLSelectElement | null = null;
  private modalOutputDevice: HTMLSelectElement | null = null;
  private modalCaptureVolume: HTMLInputElement | null = null;
  private modalCaptureVolumeValue: HTMLElement | null = null;
  private inputChannelsInfo: HTMLElement | null = null;
  private modalOutputChannel: HTMLSelectElement | null = null;
  private modalOutputVolume: HTMLInputElement | null = null;
  private modalOutputVolumeValue: HTMLElement | null = null;
  private outputChannelsInfo: HTMLElement | null = null;
  private modalSignalType: HTMLSelectElement | null = null;
  private modalSweepDuration: HTMLSelectElement | null = null;
  private modalCaptureSampleRate: HTMLElement | null = null;
  private modalCaptureBitDepth: HTMLElement | null = null;
  private modalOutputSampleRate: HTMLElement | null = null;
  private modalOutputBitDepth: HTMLElement | null = null;
  private modalSweepDurationContainer: HTMLElement | null = null;
  private capturePhaseToggle: HTMLInputElement | null = null;
  private captureSmoothingSelect: HTMLSelectElement | null = null;
  private captureCalibrationFile: HTMLInputElement | null = null;
  private captureCalibrationBtn: HTMLButtonElement | null = null;
  private captureCalibrationClear: HTMLButtonElement | null = null;
  private captureChannelSelect: HTMLSelectElement | null = null;
  
  // Records management elements
  private recordsSidebar: HTMLElement | null = null;
  private recordsList: HTMLElement | null = null;
  private recordsToggleBtn: HTMLButtonElement | null = null;
  private recordsSelectAllBtn: HTMLButtonElement | null = null;
  private recordsDeselectAllBtn: HTMLButtonElement | null = null;
  private recordsDeleteSelectedBtn: HTMLButtonElement | null = null;
  private selectedRecordIds: Set<string> = new Set();
  
  // Routing elements and state
  private inputRoutingBtn: HTMLButtonElement | null = null;
  private outputRoutingBtn: HTMLButtonElement | null = null;
  private inputRoutingMatrix: any = null; // RoutingMatrix instance
  private outputRoutingMatrix: any = null; // RoutingMatrix instance
  private inputRouting: number[] = [];
  private outputRouting: number[] = [];
  
  // Volume state (0-100)
  private captureVolume: number = 70;
  private outputVolume: number = 50;
  
  // Output device change callback
  private outputDeviceChangeCallback: ((deviceId: string) => void) | null = null;

  // Capture state
  private captureGraphRenderer: any = null; // Will be imported dynamically
  private currentCaptureData: {
    frequencies: number[];
    rawMagnitudes: number[];
    smoothedMagnitudes: number[];
    rawPhase: number[];
    smoothedPhase: number[];
    metadata: any;
    channelData?: {
      left?: {
        rawMagnitudes: number[];
        smoothedMagnitudes?: number[];
        rawPhase?: number[];
        smoothedPhase?: number[];
      };
      right?: {
        rawMagnitudes: number[];
        smoothedMagnitudes?: number[];
        rawPhase?: number[];
        smoothedPhase?: number[];
      };
      average?: {
        rawMagnitudes: number[];
        smoothedMagnitudes?: number[];
        rawPhase?: number[];
        smoothedPhase?: number[];
      };
    };
    outputChannel?: 'left' | 'right' | 'both' | 'default';
  } | null = null;

  // Callbacks for external interactions
  private onCaptureComplete?: (
    frequencies: number[],
    magnitudes: number[],
  ) => void;

  constructor() {
    this.initializeElements();
    this.setupEventListeners();
    this.setupUIInteractions();
    this.setupModalEventListeners();
    this.setupResizer();
    this.initializeAudioDevices();
  }

  private initializeElements(): void {
    this.form = document.getElementById("autoeq_form") as HTMLFormElement;
    this.optimizeBtn = document.getElementById(
      "optimize_btn",
    ) as HTMLButtonElement;
    this.resetBtn = document.getElementById("reset_btn") as HTMLButtonElement;
    this.progressElement = document.getElementById(
      "optimization_progress",
    ) as HTMLElement;
    // Scores are now always visible in the bottom row
    this.errorElement = document.getElementById("error_display") as HTMLElement;

    // Initialize modal elements
    this.optimizationModal = document.getElementById(
      "optimization_modal",
    ) as HTMLElement;
    this.progressStatus = document.getElementById(
      "progress_status",
    ) as HTMLElement;
    this.elapsedTimeElement = document.getElementById(
      "elapsed_time",
    ) as HTMLElement;
    this.progressTableBody = document.getElementById(
      "progress_table_body",
    ) as HTMLElement;

    // Debug element initialization
    console.log("[UI INIT] Modal elements found:");
    console.log("  optimizationModal:", !!this.optimizationModal);
    console.log("  progressStatus:", !!this.progressStatus);
    console.log("  elapsedTimeElement:", !!this.elapsedTimeElement);
    console.log("  progressTableBody:", !!this.progressTableBody);
    this.cancelOptimizationBtn = document.getElementById(
      "cancel_optimization",
    ) as HTMLButtonElement;
    this.doneOptimizationBtn = document.getElementById(
      "done_optimization",
    ) as HTMLButtonElement;
    this.modalCloseBtn = document.getElementById(
      "modal_close",
    ) as HTMLButtonElement;
    this.progressGraphElement = document.getElementById(
      "progress_graph",
    ) as HTMLElement;

    // Initialize audio elements
    this.demoAudioSelect = document.getElementById(
      "demo_audio_select",
    ) as HTMLSelectElement;
    this.eqOnBtn = document.getElementById("eq_on_btn") as HTMLButtonElement;
    this.eqOffBtn = document.getElementById("eq_off_btn") as HTMLButtonElement;
    this.listenBtn = document.getElementById("listen_btn") as HTMLButtonElement;
    console.log("Listen button found:", this.listenBtn);
    console.log("Listen button initial state:", {
      id: this.listenBtn?.id,
      className: this.listenBtn?.className,
      disabled: this.listenBtn?.disabled,
      tagName: this.listenBtn?.tagName,
    });

    // Check for duplicate elements
    const allListenButtons = document.querySelectorAll("#listen_btn");
    const allListenButtonsByClass = document.querySelectorAll(".listen-button");
    console.log("Total elements with ID listen_btn:", allListenButtons.length);
    console.log(
      "Total elements with class listen-button:",
      allListenButtonsByClass.length,
    );
    if (allListenButtons.length > 1) {
      console.warn(
        "Multiple elements found with ID listen_btn!",
        allListenButtons,
      );
    }

    // Add debugging to track what's disabling the button
    if (this.listenBtn) {
      const originalDisabledSetter = Object.getOwnPropertyDescriptor(
        HTMLButtonElement.prototype,
        "disabled",
      )?.set;
      if (originalDisabledSetter) {
        Object.defineProperty(this.listenBtn, "disabled", {
          set: function (value: boolean) {
            console.log(
              `Listen button disabled property being set to: ${value}`,
            );
            console.trace("Stack trace for disabled setter:");
            originalDisabledSetter.call(this, value);
          },
          get: function () {
            return this.hasAttribute("disabled");
          },
          configurable: true,
        });
      }
    }
    this.stopBtn = document.getElementById("stop_btn") as HTMLButtonElement;
    this.audioStatus = document.getElementById("audio_status") as HTMLElement;
    this.audioStatusText = document.getElementById(
      "audio_status_text",
    ) as HTMLElement;
    this.audioDuration = document.getElementById(
      "audio_duration",
    ) as HTMLElement;
    this.audioPosition = document.getElementById(
      "audio_position",
    ) as HTMLElement;
    this.audioProgressFill = document.getElementById(
      "audio_progress_fill",
    ) as HTMLElement;

    // Capture elements
    this.captureBtn = document.getElementById(
      "capture_btn",
    ) as HTMLButtonElement;
    this.captureStatus = document.getElementById(
      "capture_status",
    ) as HTMLElement;
    this.captureStatusText = document.getElementById(
      "capture_status_text",
    ) as HTMLElement;
    this.captureProgressFill = document.getElementById(
      "capture_progress_fill",
    ) as HTMLElement;
    this.captureWaveform = document.getElementById(
      "capture_waveform",
    ) as HTMLCanvasElement;
    this.captureWaveformCtx = this.captureWaveform
      ? this.captureWaveform.getContext("2d")
      : null;
    this.captureResult = document.getElementById(
      "capture_result",
    ) as HTMLElement;
    this.captureClearBtn = document.getElementById(
      "capture_clear",
    ) as HTMLButtonElement;
    this.capturePlot = document.getElementById("capture_plot") as HTMLElement;
    this.captureDeviceSelect = document.getElementById(
      "capture_device",
    ) as HTMLSelectElement;
    this.sweepDurationSelect = document.getElementById(
      "sweep_duration",
    ) as HTMLSelectElement;
    this.outputChannelSelect = document.getElementById(
      "output_channel",
    ) as HTMLSelectElement;
    this.captureSampleRateSelect = document.getElementById(
      "capture_sample_rate",
    ) as HTMLSelectElement;
    this.signalTypeSelect = document.getElementById(
      "signal_type",
    ) as HTMLSelectElement;

    // Initialize capture modal elements
    this.captureModal = document.getElementById("capture_modal");
    this.captureModalGraph = document.getElementById(
      "capture_modal_graph",
    ) as HTMLCanvasElement;
    this.captureModalPlaceholder = document.getElementById(
      "capture_modal_placeholder",
    );
    this.captureModalProgress = document.getElementById(
      "capture_modal_progress",
    );
    this.captureModalProgressFill = document.getElementById(
      "capture_modal_progress_fill",
    );
    this.captureModalStatus = document.getElementById(
      "capture_modal_status",
    );
    this.captureModalStart = document.getElementById(
      "capture_modal_start",
    ) as HTMLButtonElement;
    this.captureModalStop = document.getElementById(
      "capture_modal_stop",
    ) as HTMLButtonElement;
    this.captureModalExport = document.getElementById(
      "capture_modal_export",
    ) as HTMLButtonElement;
    this.captureModalCancel = document.getElementById(
      "capture_modal_cancel",
    ) as HTMLButtonElement;
    this.captureModalClose = document.getElementById(
      "capture_modal_close",
    ) as HTMLButtonElement;

    // Modal capture control elements
    this.modalCaptureDevice = document.getElementById(
      "modal_capture_device",
    ) as HTMLSelectElement;
    this.modalOutputDevice = document.getElementById(
      "modal_output_device",
    ) as HTMLSelectElement;
    this.modalCaptureVolume = document.getElementById(
      "modal_capture_volume",
    ) as HTMLInputElement;
    this.modalCaptureVolumeValue = document.getElementById(
      "modal_capture_volume_value",
    );
    this.inputChannelsInfo = document.getElementById(
      "input_channels_info",
    );
    this.modalOutputChannel = document.getElementById(
      "modal_output_channel",
    ) as HTMLSelectElement;
    this.modalOutputVolume = document.getElementById(
      "modal_output_volume",
    ) as HTMLInputElement;
    this.modalOutputVolumeValue = document.getElementById(
      "modal_output_volume_value",
    );
    this.outputChannelsInfo = document.getElementById(
      "output_channels_info",
    );
    this.modalSignalType = document.getElementById(
      "modal_signal_type",
    ) as HTMLSelectElement;
    this.modalSweepDuration = document.getElementById(
      "modal_sweep_duration",
    ) as HTMLSelectElement;
    this.modalCaptureSampleRate = document.getElementById(
      "modal_capture_sample_rate",
    ) as HTMLElement;
    this.modalCaptureBitDepth = document.getElementById(
      "modal_capture_bit_depth",
    ) as HTMLElement;
    this.modalOutputSampleRate = document.getElementById(
      "modal_output_sample_rate",
    ) as HTMLElement;
    this.modalOutputBitDepth = document.getElementById(
      "modal_output_bit_depth",
    ) as HTMLElement;
    this.modalSweepDurationContainer = document.getElementById(
      "modal_sweep_duration_container",
    );
    this.capturePhaseToggle = document.getElementById(
      "capture_phase_toggle",
    ) as HTMLInputElement;
    this.captureSmoothingSelect = document.getElementById(
      "capture_smoothing_select",
    ) as HTMLSelectElement;
    this.captureCalibrationFile = document.getElementById(
      "capture_calibration_file",
    ) as HTMLInputElement;
    this.captureCalibrationBtn = document.getElementById(
      "capture_calibration_btn",
    ) as HTMLButtonElement;
    this.captureCalibrationClear = document.getElementById(
      "capture_calibration_clear",
    ) as HTMLButtonElement;
    this.captureChannelSelect = document.getElementById(
      "capture_channel_select",
    ) as HTMLSelectElement;
    
    // Records management elements
    this.recordsSidebar = document.getElementById("capture_records_sidebar");
    this.recordsList = document.getElementById("capture_records_list");
    this.recordsToggleBtn = document.getElementById(
      "records_toggle",
    ) as HTMLButtonElement;
    this.recordsSelectAllBtn = document.getElementById(
      "records_select_all",
    ) as HTMLButtonElement;
    this.recordsDeselectAllBtn = document.getElementById(
      "records_deselect_all",
    ) as HTMLButtonElement;
    this.recordsDeleteSelectedBtn = document.getElementById(
      "records_delete_selected",
    ) as HTMLButtonElement;
    
    // Routing buttons
    this.inputRoutingBtn = document.getElementById(
      "input_routing_btn",
    ) as HTMLButtonElement;
    this.outputRoutingBtn = document.getElementById(
      "output_routing_btn",
    ) as HTMLButtonElement;
  }

  private setupEventListeners(): void {
    // Form submission
    this.form.addEventListener("submit", (e) => {
      e.preventDefault();
      this.onOptimizeClick();
    });

    // Reset button
    this.resetBtn.addEventListener("click", () => {
      this.resetToDefaults();
    });

    // Capture button (opens modal)
    this.captureBtn?.addEventListener("click", () => {
      this.openCaptureModal();
    });

    // Clear capture button
    this.captureClearBtn?.addEventListener("click", () => {
      this.clearCaptureResults();
    });

    // Sweep duration selector
    this.sweepDurationSelect?.addEventListener("change", () => {
      console.log(
        "Sweep duration changed to:",
        this.sweepDurationSelect?.value,
      );
    });

    // Output channel selector
    this.outputChannelSelect?.addEventListener("change", () => {
      console.log(
        "Output channel changed to:",
        this.outputChannelSelect?.value,
      );
    });

    // Sample rate selector
    this.captureSampleRateSelect?.addEventListener("change", () => {
      console.log(
        "Sample rate changed to:",
        this.captureSampleRateSelect?.value,
      );
    });

    // Signal type selector
    this.signalTypeSelect?.addEventListener("change", () => {
      const signalType = this.signalTypeSelect?.value;
      console.log("Signal type changed to:", signalType);

      // Show/hide sweep duration based on signal type
      const durationContainer = document.getElementById(
        "sweep_duration_container",
      );
      if (durationContainer) {
        durationContainer.style.display =
          signalType === "sweep" ? "flex" : "none";
      }
    });

    // Audio control buttons
    this.eqOnBtn?.addEventListener("click", () => this.setEQEnabled(true));
    this.eqOffBtn?.addEventListener("click", () => this.setEQEnabled(false));
    this.listenBtn?.addEventListener("click", () => this.onListenClick());
    this.stopBtn?.addEventListener("click", () => this.onStopClick());

    // Capture modal event listeners
    this.setupCaptureModalEventListeners();
  }

  private setupUIInteractions(): void {
    // Algorithm change handler
    const algoSelect = document.getElementById("algo") as HTMLSelectElement;
    if (algoSelect) {
      algoSelect.addEventListener("change", () => {
        this.updateConditionalParameters();
      });
    }

    // Input source change handler and tab switching
    const inputSourceRadios = document.querySelectorAll(
      'input[name="input_source"]',
    );
    inputSourceRadios.forEach((radio) => {
      radio.addEventListener("change", (e) => {
        const target = e.target as HTMLInputElement;
        const value = target.value;

        // Update conditional parameters
        this.updateConditionalParameters();

        // Handle tab switching
        this.switchTab(value);
      });
    });

    // Tab label click handlers
    const tabLabels = document.querySelectorAll(".tab-label");
    tabLabels.forEach((label) => {
      label.addEventListener("click", (e) => {
        const tabName = (e.currentTarget as HTMLElement).getAttribute(
          "data-tab",
        );
        if (tabName) {
          // Find and check the corresponding radio button
          const radio = document.querySelector(
            `input[name="input_source"][value="${tabName}"]`,
          ) as HTMLInputElement;
          if (radio) {
            radio.checked = true;
            this.switchTab(tabName);
            this.updateConditionalParameters();
          }
        }
      });
    });

    // Grid layout - accordion functionality removed
  }

  private setupModalEventListeners(): void {
    // Modal close handlers
    this.modalCloseBtn?.addEventListener("click", () => {
      this.closeOptimizationModal();
    });

    this.doneOptimizationBtn?.addEventListener("click", () => {
      this.closeOptimizationModal();
    });

    // Cancel optimization
    this.cancelOptimizationBtn?.addEventListener("click", () => {
      this.cancelOptimization();
    });

    // Close modal when clicking outside
    this.optimizationModal?.addEventListener("click", (e) => {
      if (e.target === this.optimizationModal) {
        this.closeOptimizationModal();
      }
    });
  }

  private setupCaptureModalEventListeners(): void {
    // Modal close handlers
    this.captureModalClose?.addEventListener("click", () => {
      this.closeCaptureModal();
    });

    this.captureModalCancel?.addEventListener("click", () => {
      this.closeCaptureModal();
    });

    // Capture control buttons
    this.captureModalStart?.addEventListener("click", async () => {
      await this.startModalCapture();
    });

    this.captureModalStop?.addEventListener("click", () => {
      this.stopModalCapture();
    });

    this.captureModalExport?.addEventListener("click", () => {
      this.exportCaptureCSV();
    });

    // Signal type change handler for modal
    this.modalSignalType?.addEventListener("change", () => {
      const signalType = this.modalSignalType?.value;
      console.log("Modal signal type changed to:", signalType);
      
      // Show/hide sweep duration based on signal type
      if (this.modalSweepDurationContainer) {
        this.modalSweepDurationContainer.style.display =
          signalType === "sweep" ? "flex" : "none";
      }
    });

    // Output channel change handler for modal
    this.modalOutputChannel?.addEventListener("change", () => {
      const outputChannel = this.modalOutputChannel?.value || "both";
      console.log("Modal output channel changed to:", outputChannel);
      
      // Update channel select options based on output channel
      this.updateChannelSelectOptions(outputChannel);
      
      // Update sample rate options based on selected device
      this.updateSampleRateForDevice();
    });

    // Device change handler for modal
    this.modalCaptureDevice?.addEventListener("change", async () => {
      const deviceId = this.modalCaptureDevice?.value || "default";
      console.log("Modal capture device changed to:", deviceId);
      
      // Update sample rate and bit depth based on selected device
      await this.updateSampleRateForDevice();
      
      // Update output channel options based on device capabilities
      await this.updateOutputChannelOptions();
    });
    
    // Output device change handler for modal
    this.modalOutputDevice?.addEventListener("change", async () => {
      const deviceId = this.modalOutputDevice?.value || "default";
      console.log("Modal output device changed to:", deviceId);
      
      // Update output device channel info
      await this.updateOutputDeviceInfo();
      
      // Notify audio player about output device change
      if (this.outputDeviceChangeCallback) {
        this.outputDeviceChangeCallback(deviceId);
      }
    });
    
    // Volume slider handlers
    this.modalCaptureVolume?.addEventListener("input", () => {
      this.onVolumeChange();
    });
    
    this.modalOutputVolume?.addEventListener("input", () => {
      this.onOutputVolumeChange();
    });

    // Close modal when clicking outside
    this.captureModal?.addEventListener("click", (e) => {
      if (e.target === this.captureModal) {
        this.closeCaptureModal();
      }
    });

    // ESC key to close modal
    document.addEventListener("keydown", (e) => {
      if (e.key === "Escape" && this.captureModal?.style.display === "flex") {
        this.closeCaptureModal();
      }
    });

    // Phase toggle handler
    this.capturePhaseToggle?.addEventListener("change", () => {
      this.onPhaseToggleChange();
    });

    // Smoothing selector handler
    this.captureSmoothingSelect?.addEventListener("change", () => {
      this.onSmoothingChange();
    });

    // Calibration file handlers
    this.captureCalibrationBtn?.addEventListener("click", () => {
      this.captureCalibrationFile?.click();
    });

    this.captureCalibrationFile?.addEventListener("change", (e) => {
      this.onCalibrationFileChange(e);
    });

    this.captureCalibrationClear?.addEventListener("click", () => {
      this.clearCalibrationFile();
    });

    // Channel visibility control
    this.captureChannelSelect?.addEventListener("change", () => {
      this.onChannelDisplayChange();
    });
    
    // Records panel event handlers
    this.recordsToggleBtn?.addEventListener("click", () => {
      this.toggleRecordsSidebar();
    });
    
    this.recordsSelectAllBtn?.addEventListener("click", () => {
      this.selectAllRecords();
    });
    
    this.recordsDeselectAllBtn?.addEventListener("click", () => {
      this.deselectAllRecords();
    });
    
    this.recordsDeleteSelectedBtn?.addEventListener("click", () => {
      this.deleteSelectedRecords();
    });
  }

  private onVolumeChange(): void {
    if (!this.modalCaptureVolume || !this.modalCaptureVolumeValue) return;
    
    const volume = parseInt(this.modalCaptureVolume.value);
    this.captureVolume = volume;
    
    // Update display value
    this.modalCaptureVolumeValue.textContent = `${volume}%`;
    
    // Update slider gradient to show filled portion
    const percentage = volume;
    this.modalCaptureVolume.style.background = `linear-gradient(to right, 
      var(--button-primary) 0%, 
      var(--button-primary) ${percentage}%, 
      var(--bg-accent) ${percentage}%, 
      var(--bg-accent) 100%)`;
    
    console.log(`Input volume changed to: ${volume}%`);
  }
  
  private onOutputVolumeChange(): void {
    if (!this.modalOutputVolume || !this.modalOutputVolumeValue) return;
    
    const volume = parseInt(this.modalOutputVolume.value);
    this.outputVolume = volume;
    
    // Update display value
    this.modalOutputVolumeValue.textContent = `${volume}%`;
    
    // Update slider gradient to show filled portion
    const percentage = volume;
    this.modalOutputVolume.style.background = `linear-gradient(to right, 
      var(--button-primary) 0%, 
      var(--button-primary) ${percentage}%, 
      var(--bg-accent) ${percentage}%, 
      var(--bg-accent) 100%)`;
    
    console.log(`Output volume changed to: ${volume}%`);
  }
  
  private onPhaseToggleChange(): void {
    if (this.captureGraphRenderer && this.capturePhaseToggle) {
      const showPhase = this.capturePhaseToggle.checked;
      this.captureGraphRenderer.setPhaseVisibility(showPhase);
      
      // Re-render the graph with the current data if available
      if (this.currentCaptureData) {
        this.captureGraphRenderer.renderGraph({
          frequencies: this.currentCaptureData.frequencies,
          rawMagnitudes: this.currentCaptureData.rawMagnitudes,
          smoothedMagnitudes: this.currentCaptureData.smoothedMagnitudes,
          rawPhase: this.currentCaptureData.rawPhase,
          smoothedPhase: this.currentCaptureData.smoothedPhase
        });
      } else {
        // Re-render placeholder
        this.captureGraphRenderer.renderPlaceholder();
      }
    }
  }

  private onSmoothingChange(): void {
    // Re-process the capture data with new smoothing settings if we have raw data
    if (this.currentCaptureData && this.captureSmoothingSelect) {
      const octaveFraction = parseInt(this.captureSmoothingSelect.value);
      console.log("Smoothing changed to 1/" + octaveFraction + " octave");
      
      // Re-smooth the data with new settings
      this.reprocessCaptureData(octaveFraction);
    }
  }

  private async reprocessCaptureData(octaveFraction: number): Promise<void> {
    if (!this.currentCaptureData) return;
    
    try {
      const { CaptureGraphRenderer } = await import("./audio/capture-graph");
      
      // Re-apply smoothing with new octave fraction to main data
      const smoothedMagnitudes = CaptureGraphRenderer.applySmoothing(
        this.currentCaptureData.frequencies,
        this.currentCaptureData.rawMagnitudes,
        octaveFraction
      );
      
      let smoothedPhase: number[] = [];
      if (this.currentCaptureData.rawPhase.length > 0) {
        smoothedPhase = CaptureGraphRenderer.applyPhaseSmoothing(
          this.currentCaptureData.frequencies,
          this.currentCaptureData.rawPhase,
          octaveFraction
        );
      }
      
      // Update the stored data
      this.currentCaptureData.smoothedMagnitudes = smoothedMagnitudes;
      this.currentCaptureData.smoothedPhase = smoothedPhase;
      
      // Re-smooth channel-specific data if it exists
      if (this.currentCaptureData.channelData) {
        // Left channel
        if (this.currentCaptureData.channelData.left) {
          this.currentCaptureData.channelData.left.smoothedMagnitudes = CaptureGraphRenderer.applySmoothing(
            this.currentCaptureData.frequencies,
            this.currentCaptureData.channelData.left.rawMagnitudes,
            octaveFraction
          );
          if (this.currentCaptureData.channelData.left.rawPhase && this.currentCaptureData.channelData.left.rawPhase.length > 0) {
            this.currentCaptureData.channelData.left.smoothedPhase = CaptureGraphRenderer.applyPhaseSmoothing(
              this.currentCaptureData.frequencies,
              this.currentCaptureData.channelData.left.rawPhase,
              octaveFraction
            );
          }
        }
        
        // Right channel
        if (this.currentCaptureData.channelData.right) {
          this.currentCaptureData.channelData.right.smoothedMagnitudes = CaptureGraphRenderer.applySmoothing(
            this.currentCaptureData.frequencies,
            this.currentCaptureData.channelData.right.rawMagnitudes,
            octaveFraction
          );
          if (this.currentCaptureData.channelData.right.rawPhase && this.currentCaptureData.channelData.right.rawPhase.length > 0) {
            this.currentCaptureData.channelData.right.smoothedPhase = CaptureGraphRenderer.applyPhaseSmoothing(
              this.currentCaptureData.frequencies,
              this.currentCaptureData.channelData.right.rawPhase,
              octaveFraction
            );
          }
        }
        
        // Average channel
        if (this.currentCaptureData.channelData.average) {
          this.currentCaptureData.channelData.average.smoothedMagnitudes = CaptureGraphRenderer.applySmoothing(
            this.currentCaptureData.frequencies,
            this.currentCaptureData.channelData.average.rawMagnitudes,
            octaveFraction
          );
          if (this.currentCaptureData.channelData.average.rawPhase && this.currentCaptureData.channelData.average.rawPhase.length > 0) {
            this.currentCaptureData.channelData.average.smoothedPhase = CaptureGraphRenderer.applyPhaseSmoothing(
              this.currentCaptureData.frequencies,
              this.currentCaptureData.channelData.average.rawPhase,
              octaveFraction
            );
          }
        }
      }
      
      // Re-render the graph with updated channel data
      if (this.captureGraphRenderer) {
        this.captureGraphRenderer.renderGraph({
          frequencies: this.currentCaptureData.frequencies,
          rawMagnitudes: this.currentCaptureData.rawMagnitudes,
          smoothedMagnitudes: this.currentCaptureData.smoothedMagnitudes,
          rawPhase: this.currentCaptureData.rawPhase,
          smoothedPhase: this.currentCaptureData.smoothedPhase,
          channelData: this.currentCaptureData.channelData,
          outputChannel: this.currentCaptureData.outputChannel
        });
      }
    } catch (error) {
      console.error("Error reprocessing capture data:", error);
    }
  }

  private async onChannelDisplayChange(): Promise<void> {
    if (!this.captureChannelSelect || !this.captureGraphRenderer) {
      console.warn('Channel select or graph renderer not available');
      return;
    }
    
    const selectedDisplay = this.captureChannelSelect.value;
    console.log(`Channel display changed to: ${selectedDisplay}`);
    
    try {
      // Handle special cases first (these don't require currentCaptureData)
      if (selectedDisplay === 'lr_sum') {
        await this.displayLRSum();
        return;
      }
      
      if (selectedDisplay === 'combined_all') {
        await this.displayCombinedAll();
        return;
      }
      
      // Handle specific capture selection (these don't require currentCaptureData)
      if (selectedDisplay.startsWith('capture_')) {
        const captureId = selectedDisplay.replace('capture_', '');
        console.log(`Loading stored capture: ${captureId}`);
        await this.displayStoredCapture(captureId);
        return;
      }
      
      // For other options, we need current capture data
      if (!this.currentCaptureData) {
        console.warn('No current capture data available for this display option');
        this.captureGraphRenderer.renderPlaceholder();
        return;
      }
      
      // Handle current capture with channel visibility
      if (selectedDisplay === 'current') {
        console.log('Showing current capture (combined)');
        // Show current capture with combined channel
        this.captureGraphRenderer.setChannelVisibility('combined', true);
        this.captureGraphRenderer.setChannelVisibility('left', false);
        this.captureGraphRenderer.setChannelVisibility('right', false);
        this.captureGraphRenderer.setChannelVisibility('average', false);
      } else if (selectedDisplay === 'average') {
        console.log('Showing average channel');
        this.captureGraphRenderer.setChannelVisibility('combined', false);
        this.captureGraphRenderer.setChannelVisibility('left', false);
        this.captureGraphRenderer.setChannelVisibility('right', false);
        this.captureGraphRenderer.setChannelVisibility('average', true);
      } else if (selectedDisplay === 'left') {
        console.log('Showing left channel');
        this.captureGraphRenderer.setChannelVisibility('combined', false);
        this.captureGraphRenderer.setChannelVisibility('left', true);
        this.captureGraphRenderer.setChannelVisibility('right', false);
        this.captureGraphRenderer.setChannelVisibility('average', false);
      } else if (selectedDisplay === 'right') {
        console.log('Showing right channel');
        this.captureGraphRenderer.setChannelVisibility('combined', false);
        this.captureGraphRenderer.setChannelVisibility('left', false);
        this.captureGraphRenderer.setChannelVisibility('right', true);
        this.captureGraphRenderer.setChannelVisibility('average', false);
      } else if (selectedDisplay === 'all') {
        console.log('Showing all channels');
        this.captureGraphRenderer.setChannelVisibility('combined', true);
        this.captureGraphRenderer.setChannelVisibility('left', true);
        this.captureGraphRenderer.setChannelVisibility('right', true);
        this.captureGraphRenderer.setChannelVisibility('average', true);
      } else {
        console.warn(`Unknown display option: ${selectedDisplay}`);
        return;
      }
      
      // Re-render current data
      console.log('Rendering graph with channelData:', !!this.currentCaptureData.channelData);
      this.captureGraphRenderer.renderGraph({
        frequencies: this.currentCaptureData.frequencies,
        rawMagnitudes: this.currentCaptureData.rawMagnitudes,
        smoothedMagnitudes: this.currentCaptureData.smoothedMagnitudes,
        rawPhase: this.currentCaptureData.rawPhase,
        smoothedPhase: this.currentCaptureData.smoothedPhase,
        channelData: this.currentCaptureData.channelData,
        outputChannel: this.currentCaptureData.outputChannel
      });
    } catch (error) {
      console.error('Error changing channel display:', error);
    }
  }
  
  /**
   * Display L+R average computed from separate left and right captures
   */
  private async displayLRSum(): Promise<void> {
    try {
      const { CaptureStorage } = await import('./audio/capture-storage');
      const { averageLeftRightChannels } = await import('./audio/audio-math');
      
      const captureOptions = await this.getAvailableCaptureOptions();
      
      if (!captureOptions.leftCapture || !captureOptions.rightCapture) {
        console.error('Cannot compute L+R sum: missing left or right capture');
        return;
      }
      
      const leftCapture = captureOptions.leftCapture;
      const rightCapture = captureOptions.rightCapture;
      
      // Prepare frequency responses for complex addition
      const leftResponse = {
        frequencies: leftCapture.frequencies,
        magnitudes: leftCapture.smoothedMagnitudes,
        phases: leftCapture.smoothedPhase
      };
      
      const rightResponse = {
        frequencies: rightCapture.frequencies,
        magnitudes: rightCapture.smoothedMagnitudes,
        phases: rightCapture.smoothedPhase
      };
      
      // Perform complex average
      const avgResult = averageLeftRightChannels(leftResponse, rightResponse);
      
      if (!avgResult) {
        console.error('Failed to compute L+R average');
        return;
      }
      
      // Display the result
      this.captureGraphRenderer.setChannelVisibility('combined', true);
      this.captureGraphRenderer.setChannelVisibility('left', false);
      this.captureGraphRenderer.setChannelVisibility('right', false);
      this.captureGraphRenderer.setChannelVisibility('average', false);
      
      this.captureGraphRenderer.renderGraph({
        frequencies: avgResult.frequencies,
        rawMagnitudes: avgResult.magnitudes,
        smoothedMagnitudes: avgResult.magnitudes,
        rawPhase: avgResult.phases,
        smoothedPhase: avgResult.phases
      });
      
      console.log('L+R Average displayed successfully');
    } catch (error) {
      console.error('Error displaying L+R average:', error);
    }
  }
  
  /**
   * Display selected captures combined (overlaid on the same graph)
   */
  private async displayCombinedAll(): Promise<void> {
    try {
      const { CaptureStorage } = await import('./audio/capture-storage');
      
      // Get selected captures
      const selectedCaptures = [];
      for (const captureId of this.selectedRecordIds) {
        const capture = CaptureStorage.getCapture(captureId);
        if (capture) {
          selectedCaptures.push(capture);
        }
      }
      
      if (selectedCaptures.length === 0) {
        console.log('No selected captures to display, showing all');
        const allCaptures = CaptureStorage.getAllCaptures();
        selectedCaptures.push(...allCaptures);
      }
      
      if (selectedCaptures.length === 0) {
        console.error('No captures available to display');
        return;
      }
      
      // Render multiple captures on the graph
      await this.renderMultipleCaptures(selectedCaptures);
      
      console.log(`Combined (All) displayed: showing ${selectedCaptures.length} capture(s)`);
    } catch (error) {
      console.error('Error displaying combined all:', error);
    }
  }
  
  /**
   * Display a specific stored capture by ID
   */
  private async displayStoredCapture(captureId: string): Promise<void> {
    try {
      const { CaptureStorage } = await import('./audio/capture-storage');
      const capture = CaptureStorage.getCapture(captureId);
      
      if (!capture) {
        console.error(`Capture ${captureId} not found`);
        return;
      }
      
      this.captureGraphRenderer.setChannelVisibility('combined', true);
      this.captureGraphRenderer.setChannelVisibility('left', false);
      this.captureGraphRenderer.setChannelVisibility('right', false);
      this.captureGraphRenderer.setChannelVisibility('average', false);
      
      this.captureGraphRenderer.renderGraph({
        frequencies: capture.frequencies,
        rawMagnitudes: capture.rawMagnitudes,
        smoothedMagnitudes: capture.smoothedMagnitudes,
        rawPhase: capture.rawPhase,
        smoothedPhase: capture.smoothedPhase
      });
      
      console.log(`Displayed stored capture: ${capture.name}`);
    } catch (error) {
      console.error('Error displaying stored capture:', error);
    }
  }
  
  /**
   * Render multiple captures overlaid on the graph
   */
  private async renderMultipleCaptures(captures: any[]): Promise<void> {
    if (!this.captureGraphRenderer || captures.length === 0) return;
    
    // For now, render just the first selected capture
    // Future enhancement: overlay all captures with different colors
    const firstCapture = captures[0];
    
    this.captureGraphRenderer.setChannelVisibility('combined', true);
    this.captureGraphRenderer.setChannelVisibility('left', false);
    this.captureGraphRenderer.setChannelVisibility('right', false);
    this.captureGraphRenderer.setChannelVisibility('average', false);
    
    this.captureGraphRenderer.renderGraph({
      frequencies: firstCapture.frequencies,
      rawMagnitudes: firstCapture.rawMagnitudes,
      smoothedMagnitudes: firstCapture.smoothedMagnitudes,
      rawPhase: firstCapture.rawPhase,
      smoothedPhase: firstCapture.smoothedPhase
    });
    
    // TODO: Implement true multi-capture overlay with the capture-graph renderer
    // This would require enhancing the CaptureGraphRenderer to support multiple datasets
  }
  
  /**
   * Render the records list in the sidebar
   */
  private async renderRecordsList(): Promise<void> {
    if (!this.recordsList) return;
    
    try {
      const { CaptureStorage } = await import('./audio/capture-storage');
      const captures = CaptureStorage.getAllCaptures();
      
      if (captures.length === 0) {
        this.recordsList.innerHTML = '<div class="no-records">No saved records</div>';
        return;
      }
      
      // Generate color palette for records
      const colors = [
        '#007bff', '#28a745', '#dc3545', '#ffc107', '#17a2b8',
        '#6610f2', '#e83e8c', '#fd7e14', '#20c997', '#6f42c1'
      ];
      
      this.recordsList.innerHTML = '';
      const listElement = this.recordsList; // Store reference for forEach
      
      captures.forEach((capture, index) => {
        const recordItem = document.createElement('div');
        recordItem.className = 'record-item';
        recordItem.dataset.captureId = capture.id;
        
        if (this.selectedRecordIds.has(capture.id)) {
          recordItem.classList.add('selected');
        }
        
        const color = colors[index % colors.length];
        
        recordItem.innerHTML = `
          <div class="record-color-indicator" style="background: ${color};"></div>
          <input type="checkbox" class="record-checkbox" ${this.selectedRecordIds.has(capture.id) ? 'checked' : ''}>
          <div class="record-info">
            <div class="record-name" contenteditable="false">${capture.name}</div>
            <div class="record-meta">${this.formatRecordMeta(capture)}</div>
          </div>
          <div class="record-actions">
            <button class="record-action-btn rename" title="Rename">✏️</button>
            <button class="record-action-btn delete" title="Delete">🗑️</button>
          </div>
        `;
        
        // Add event listeners
        const checkbox = recordItem.querySelector('.record-checkbox') as HTMLInputElement;
        checkbox?.addEventListener('change', (e) => {
          e.stopPropagation();
          this.toggleRecordSelection(capture.id, checkbox.checked);
        });
        
        const renameBtn = recordItem.querySelector('.rename') as HTMLButtonElement;
        renameBtn?.addEventListener('click', (e) => {
          e.stopPropagation();
          this.renameRecord(capture.id, recordItem);
        });
        
        const deleteBtn = recordItem.querySelector('.delete') as HTMLButtonElement;
        deleteBtn?.addEventListener('click', (e) => {
          e.stopPropagation();
          this.deleteRecord(capture.id);
        });
        
        // Click on item to toggle selection
        recordItem.addEventListener('click', () => {
          checkbox.checked = !checkbox.checked;
          this.toggleRecordSelection(capture.id, checkbox.checked);
        });
        
        listElement.appendChild(recordItem);
      });
    } catch (error) {
      console.error('Error rendering records list:', error);
    }
  }
  
  /**
   * Format record metadata for display
   */
  private formatRecordMeta(capture: any): string {
    const channel = capture.outputChannel === 'both' ? 'Stereo' :
                   capture.outputChannel === 'left' ? 'L' :
                   capture.outputChannel === 'right' ? 'R' : 'Mono';
    const date = new Date(capture.timestamp).toLocaleDateString();
    const time = new Date(capture.timestamp).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
    return `${channel} • ${date} ${time}`;
  }
  
  /**
   * Toggle sidebar visibility
   */
  private toggleRecordsSidebar(): void {
    if (!this.recordsSidebar || !this.recordsToggleBtn) return;
    
    const isCollapsed = this.recordsSidebar.classList.toggle('collapsed');
    this.recordsToggleBtn.textContent = isCollapsed ? '▶' : '◀';
    
    // Trigger canvas resize after transition completes
    setTimeout(() => {
      if (this.captureGraphRenderer) {
        this.captureGraphRenderer.resize();
      }
    }, 350); // Slightly longer than CSS transition (300ms)
  }
  
  /**
   * Toggle record selection
   */
  private toggleRecordSelection(captureId: string, selected: boolean): void {
    if (selected) {
      this.selectedRecordIds.add(captureId);
    } else {
      this.selectedRecordIds.delete(captureId);
    }
    
    // Update UI
    const recordItem = this.recordsList?.querySelector(`[data-capture-id="${captureId}"]`);
    if (recordItem) {
      recordItem.classList.toggle('selected', selected);
    }
    
    console.log(`Record ${captureId} ${selected ? 'selected' : 'deselected'}`);
  }
  
  /**
   * Select all records
   */
  private async selectAllRecords(): Promise<void> {
    try {
      const { CaptureStorage } = await import('./audio/capture-storage');
      const captures = CaptureStorage.getAllCaptures();
      
      captures.forEach(capture => {
        this.selectedRecordIds.add(capture.id);
      });
      
      await this.renderRecordsList();
      console.log('All records selected');
    } catch (error) {
      console.error('Error selecting all records:', error);
    }
  }
  
  /**
   * Deselect all records
   */
  private async deselectAllRecords(): Promise<void> {
    this.selectedRecordIds.clear();
    await this.renderRecordsList();
    console.log('All records deselected');
  }
  
  /**
   * Rename a record
   */
  private async renameRecord(captureId: string, recordItem: HTMLElement): Promise<void> {
    const nameElement = recordItem.querySelector('.record-name') as HTMLElement;
    if (!nameElement) return;
    
    const currentName = nameElement.textContent || '';
    nameElement.contentEditable = 'true';
    nameElement.classList.add('editing');
    nameElement.focus();
    
    // Select all text
    const range = document.createRange();
    range.selectNodeContents(nameElement);
    const selection = window.getSelection();
    selection?.removeAllRanges();
    selection?.addRange(range);
    
    const finishEdit = async () => {
      nameElement.contentEditable = 'false';
      nameElement.classList.remove('editing');
      
      const newName = nameElement.textContent?.trim() || currentName;
      
      if (newName !== currentName && newName.length > 0) {
        try {
          const { CaptureStorage } = await import('./audio/capture-storage');
          const capture = CaptureStorage.getCapture(captureId);
          
          if (capture) {
            // Update name in storage (modify the capture object directly)
            capture.name = newName;
            // Note: The name is only updated in memory, not persisted to localStorage
            // A better approach would be to add an updateCapture method to CaptureStorage
            console.log(`Renamed capture ${captureId} to: ${newName}`);
            await this.renderRecordsList();
            await this.updateChannelSelectOptions(this.currentCaptureData?.outputChannel || 'both');
          }
        } catch (error) {
          console.error('Error renaming capture:', error);
          nameElement.textContent = currentName;
        }
      } else {
        nameElement.textContent = currentName;
      }
    };
    
    nameElement.addEventListener('blur', finishEdit, { once: true });
    nameElement.addEventListener('keydown', (e) => {
      if (e.key === 'Enter') {
        e.preventDefault();
        nameElement.blur();
      } else if (e.key === 'Escape') {
        nameElement.textContent = currentName;
        nameElement.blur();
      }
    });
  }
  
  /**
   * Delete a single record
   */
  private async deleteRecord(captureId: string): Promise<void> {
    if (!confirm('Delete this capture?')) return;
    
    try {
      const { CaptureStorage } = await import('./audio/capture-storage');
      CaptureStorage.deleteCapture(captureId);
      this.selectedRecordIds.delete(captureId);
      
      await this.renderRecordsList();
      await this.updateChannelSelectOptions(this.currentCaptureData?.outputChannel || 'both');
      
      console.log(`Deleted capture: ${captureId}`);
    } catch (error) {
      console.error('Error deleting capture:', error);
    }
  }
  
  /**
   * Delete selected records
   */
  private async deleteSelectedRecords(): Promise<void> {
    if (this.selectedRecordIds.size === 0) {
      alert('No records selected');
      return;
    }
    
    if (!confirm(`Delete ${this.selectedRecordIds.size} selected record(s)?`)) return;
    
    try {
      const { CaptureStorage } = await import('./audio/capture-storage');
      
      for (const captureId of this.selectedRecordIds) {
        CaptureStorage.deleteCapture(captureId);
      }
      
      this.selectedRecordIds.clear();
      await this.renderRecordsList();
      await this.updateChannelSelectOptions(this.currentCaptureData?.outputChannel || 'both');
      
      console.log('Deleted selected captures');
    } catch (error) {
      console.error('Error deleting selected captures:', error);
    }
  }

  private async updateChannelSelectOptions(outputChannel: string): Promise<void> {
    if (!this.captureChannelSelect) return;
    
    // Clear existing options
    this.captureChannelSelect.innerHTML = '';
    
    // Get available captures from storage
    const captureOptions = await this.getAvailableCaptureOptions();
    
    // Always add the current capture as "Current" option if we have current data
    if (this.currentCaptureData) {
      const currentOption = document.createElement('option');
      currentOption.value = 'current';
      currentOption.textContent = `Current${this.getChannelDisplayName(outputChannel)}`;
      this.captureChannelSelect.appendChild(currentOption);
    }
    
    // Add "Average" option for current capture if it's stereo
    if (this.currentCaptureData && this.currentCaptureData.channelData?.average) {
      const avgOption = document.createElement('option');
      avgOption.value = 'average';
      avgOption.textContent = 'Average';
      this.captureChannelSelect.appendChild(avgOption);
    }
    
    // Add separator before stored captures if we have any
    if (captureOptions.allCaptures.length > 0) {
      const separator = document.createElement('option');
      separator.disabled = true;
      separator.textContent = '─────────────────';
      this.captureChannelSelect.appendChild(separator);
    }
    
    // Add "L+R Average" option if both left and right channel captures exist
    if (captureOptions.hasBothCaptures) {
      const lrAvgOption = document.createElement('option');
      lrAvgOption.value = 'lr_sum';
      lrAvgOption.textContent = 'L+R Average (Complex)';
      this.captureChannelSelect.appendChild(lrAvgOption);
    }
    
    // Add "Combined (All)" option if multiple captures exist
    if (captureOptions.allCaptures.length > 1) {
      const combinedAllOption = document.createElement('option');
      combinedAllOption.value = 'combined_all';
      combinedAllOption.textContent = `Combined (All ${captureOptions.allCaptures.length} captures)`;
      this.captureChannelSelect.appendChild(combinedAllOption);
    }
    
    // Add individual stored captures
    if (captureOptions.allCaptures.length > 0) {
      // Add each capture with numbering
      const selectElement = this.captureChannelSelect; // Store reference for forEach
      captureOptions.allCaptures.forEach((capture, index) => {
        const option = document.createElement('option');
        option.value = `capture_${capture.id}`;
        option.textContent = `${index + 1}. ${capture.name}`;
        selectElement.appendChild(option);
      });
    }
    
    // Set default selection
    this.captureChannelSelect.value = 'current';
    console.log(`Updated channel options for output: ${outputChannel}, found ${captureOptions.allCaptures.length} stored captures`);
  }
  
  private getChannelDisplayName(outputChannel: string): string {
    switch (outputChannel) {
      case 'left': return ' (Left)';
      case 'right': return ' (Right)';
      case 'both': return ' (Stereo)';
      case 'default': return '';
      default: return ` (${outputChannel.charAt(0).toUpperCase() + outputChannel.slice(1)})`;
    }
  }

  /**
   * Detect available captures from storage and determine what options should be shown
   */
  private async getAvailableCaptureOptions(): Promise<{
    hasLeftCapture: boolean;
    hasRightCapture: boolean;
    hasBothCaptures: boolean;
    leftCapture: any | null;
    rightCapture: any | null;
    allCaptures: any[];
  }> {
    try {
      const { CaptureStorage } = await import('./audio/capture-storage');
      const capturesByChannel = CaptureStorage.getCapturesByChannel();
      
      const leftCaptures = capturesByChannel.get('left') || [];
      const rightCaptures = capturesByChannel.get('right') || [];
      const bothCaptures = capturesByChannel.get('both') || [];
      const defaultCaptures = capturesByChannel.get('default') || [];
      
      const hasLeftCapture = leftCaptures.length > 0;
      const hasRightCapture = rightCaptures.length > 0;
      const hasBothCaptures = hasLeftCapture && hasRightCapture;
      
      // Get most recent captures
      const leftCapture = leftCaptures.length > 0 ? leftCaptures[0] : null;
      const rightCapture = rightCaptures.length > 0 ? rightCaptures[0] : null;
      
      // Get all captures
      const allCaptures = CaptureStorage.getAllCaptures();
      
      return {
        hasLeftCapture,
        hasRightCapture,
        hasBothCaptures,
        leftCapture,
        rightCapture,
        allCaptures
      };
    } catch (error) {
      console.error('Error detecting available captures:', error);
      return {
        hasLeftCapture: false,
        hasRightCapture: false,
        hasBothCaptures: false,
        leftCapture: null,
        rightCapture: null,
        allCaptures: []
      };
    }
  }

  private async updateSampleRateForDevice(): Promise<void> {
    if (!this.modalCaptureSampleRate || !this.modalCaptureDevice) return;
    
    try {
      // Get the audio context to check the current sample rate
      const audioContext = new (window.AudioContext || (window as any).webkitAudioContext)();
      const deviceSampleRate = audioContext.sampleRate;
      audioContext.close(); // Clean up
      
      console.log(`Input device sample rate: ${deviceSampleRate} Hz`);
      
      // Format sample rate for badge display
      let sampleRateText = '';
      if (deviceSampleRate >= 1000) {
        const khz = deviceSampleRate / 1000;
        sampleRateText = khz % 1 === 0 ? `${khz}kHz` : `${khz.toFixed(1)}kHz`;
      } else {
        sampleRateText = `${deviceSampleRate}Hz`;
      }
      
      // Update the sample rate badge
      this.modalCaptureSampleRate.textContent = sampleRateText;
      
      // Update bit depth badge
      if (this.modalCaptureBitDepth) {
        this.modalCaptureBitDepth.textContent = '24';
      }
      
    } catch (error) {
      console.warn('Could not determine input device sample rate:', error);
      // Fall back to a common default
      if (this.modalCaptureSampleRate) {
        this.modalCaptureSampleRate.textContent = '48kHz';
      }
      if (this.modalCaptureBitDepth) {
        this.modalCaptureBitDepth.textContent = '24';
      }
    }
  }
  
  private async updateOutputSampleRate(): Promise<void> {
    if (!this.modalOutputSampleRate) return;
    
    try {
      // Get the audio context to check the current sample rate
      const audioContext = new (window.AudioContext || (window as any).webkitAudioContext)();
      const deviceSampleRate = audioContext.sampleRate;
      audioContext.close(); // Clean up
      
      console.log(`Output device sample rate: ${deviceSampleRate} Hz`);
      
      // Format sample rate for badge display
      let sampleRateText = '';
      if (deviceSampleRate >= 1000) {
        const khz = deviceSampleRate / 1000;
        sampleRateText = khz % 1 === 0 ? `${khz}kHz` : `${khz.toFixed(1)}kHz`;
      } else {
        sampleRateText = `${deviceSampleRate}Hz`;
      }
      
      // Update the sample rate badge
      this.modalOutputSampleRate.textContent = sampleRateText;
      
      // Update bit depth badge
      if (this.modalOutputBitDepth) {
        this.modalOutputBitDepth.textContent = '24';
      }
      
    } catch (error) {
      console.warn('Could not determine output device sample rate:', error);
      // Fall back to a common default
      if (this.modalOutputSampleRate) {
        this.modalOutputSampleRate.textContent = '48kHz';
      }
      if (this.modalOutputBitDepth) {
        this.modalOutputBitDepth.textContent = '24';
      }
    }
  }
  
  private async updateOutputChannelOptions(): Promise<void> {
    if (!this.modalOutputChannel || !this.modalCaptureDevice) return;
    
    try {
      const deviceId = this.modalCaptureDevice.value;
      
      // Use the device manager if available, otherwise create a new one
      let deviceManager = (this as any).deviceManager;
      if (!deviceManager) {
        const { AudioDeviceManager } = await import('./audio/device-manager');
        deviceManager = new AudioDeviceManager(true);
        await deviceManager.enumerateDevices();
        (this as any).deviceManager = deviceManager;
      }
      
      // Get device details from the manager
      let deviceInfo: any = null;
      
      if (deviceId === 'default') {
        // Find the default device
        const defaultDevice = deviceManager.findBestDevice('input', { preferDefault: true });
        if (defaultDevice) {
          deviceInfo = {
            inputChannels: defaultDevice.channels,
            outputChannels: defaultDevice.channels, // Assume same for input/output
            deviceLabel: defaultDevice.name
          };
        }
      } else {
        // Get specific device details
        try {
          const details = await deviceManager.getDeviceDetails(deviceId);
          if (details) {
            deviceInfo = {
              inputChannels: details.channels,
              outputChannels: details.channels,
              deviceLabel: details.name
            };
          }
        } catch (e) {
          // Fallback to WebAudio detection
          const { detectDeviceCapabilities } = await import('./audio/audio-device');
          deviceInfo = await detectDeviceCapabilities(deviceId);
        }
      }
      
      if (!deviceInfo) {
        this.setDeviceStatus('error', 'Device not found or cannot be accessed');
        return;
      }
      
      // Update channel info badges
      this.updateChannelInfo(deviceInfo.inputChannels, deviceInfo.outputChannels);
      
      // Update device status to success
      this.setDeviceStatus('success', `Ready - ${deviceInfo.inputChannels} channel(s)`);
      
      // Update output channel dropdown based on channel count
      this.populateOutputChannels(deviceInfo.outputChannels);
      
    } catch (error) {
      console.error('Error updating output channel options:', error);
      this.setDeviceStatus('error', 'Error checking device');
    }
  }
  
  private updateChannelInfo(inputChannels: number, outputChannels: number): void {
    // Update input channels badge
    if (this.inputChannelsInfo) {
      this.inputChannelsInfo.textContent = `${inputChannels} ch`;
      this.inputChannelsInfo.classList.add('detected');
    }
    
    // Update output channels badge
    if (this.outputChannelsInfo) {
      this.outputChannelsInfo.textContent = `${outputChannels} ch`;
      this.outputChannelsInfo.classList.add('detected');
    }
    
    // Initialize or update routing matrices
    this.initializeRoutingMatrices(inputChannels, outputChannels);
  }
  
  private async updateOutputDeviceInfo(): Promise<void> {
    if (!this.modalOutputDevice) return;
    
    try {
      const deviceId = this.modalOutputDevice.value || 'default';
      
      // For now, we'll detect output channels using the same method as inputs
      const { detectDeviceCapabilities } = await import('./audio/audio-device');
      const deviceInfo = await detectDeviceCapabilities(deviceId);
      
      if (deviceInfo) {
        // Update output channel badge
        if (this.outputChannelsInfo) {
          this.outputChannelsInfo.textContent = `${deviceInfo.outputChannels} ch`;
          this.outputChannelsInfo.classList.add('detected');
        }
        
        // Update output sample rate and bit depth badges
        await this.updateOutputSampleRate();
        
        // Update output routing matrix
        if (this.outputRoutingMatrix) {
          this.outputRoutingMatrix.updateChannelCount(deviceInfo.outputChannels);
        } else {
          // Initialize routing matrix if not already created
          await this.initializeOutputRouting(deviceInfo.outputChannels);
        }
        
        // Show/hide routing button
        if (this.outputRoutingBtn) {
          this.outputRoutingBtn.style.display = deviceInfo.outputChannels > 1 ? 'inline-flex' : 'none';
        }
        
        // Update device status to success
        this.setDeviceStatus('success', `Ready - ${deviceInfo.outputChannels} channel(s)`, 'output');
        
        console.log(`Output device updated: ${deviceInfo.outputChannels} channels`);
      } else {
        this.setDeviceStatus('error', 'Device not found', 'output');
      }
    } catch (error) {
      console.error('Error updating output device info:', error);
      this.setDeviceStatus('error', 'Error checking device', 'output');
    }
  }
  
  private async initializeOutputRouting(outputChannels: number): Promise<void> {
    try {
      const { RoutingMatrix, createRoutingButton } = await import('./audio/audio-routing');
      
      if (this.outputRoutingBtn) {
        // Add SVG icon to button
        const icon = createRoutingButton();
        this.outputRoutingBtn.innerHTML = '';
        this.outputRoutingBtn.appendChild(icon.querySelector('svg')!);
        
        // Create routing matrix instance
        this.outputRoutingMatrix = new RoutingMatrix(outputChannels);
        this.outputRoutingMatrix.setOnRoutingChange((routing: number[]) => {
          this.outputRouting = routing;
          console.log('Output routing updated:', routing);
        });
        
        // Add click handler to show matrix
        this.outputRoutingBtn.addEventListener('click', () => {
          if (this.outputRoutingMatrix && this.outputRoutingBtn) {
            this.outputRoutingMatrix.show(this.outputRoutingBtn);
          }
        });
        
        // Store initial routing
        this.outputRouting = this.outputRoutingMatrix.getRouting();
      }
    } catch (error) {
      console.error('Error initializing output routing:', error);
    }
  }
  
  private async initializeRoutingMatrices(inputChannels: number, outputChannels: number): Promise<void> {
    try {
      const { RoutingMatrix, createRoutingButton } = await import('./audio/audio-routing');
      
      // Initialize input routing
      if (this.inputRoutingBtn) {
        // Update or create routing matrix
        if (this.inputRoutingMatrix) {
          this.inputRoutingMatrix.updateChannelCount(inputChannels);
        } else {
          // Add SVG icon to button
          const icon = createRoutingButton();
          this.inputRoutingBtn.innerHTML = '';
          this.inputRoutingBtn.appendChild(icon.querySelector('svg')!);
          
          // Create routing matrix instance
          this.inputRoutingMatrix = new RoutingMatrix(inputChannels);
          this.inputRoutingMatrix.setOnRoutingChange((routing: number[]) => {
            this.inputRouting = routing;
            console.log('Input routing updated:', routing);
          });
          
          // Add click handler to show matrix
          this.inputRoutingBtn.addEventListener('click', () => {
            if (this.inputRoutingMatrix && this.inputRoutingBtn) {
              this.inputRoutingMatrix.show(this.inputRoutingBtn);
            }
          });
        }
        
        // Show button if we have more than 1 channel
        this.inputRoutingBtn.style.display = inputChannels > 1 ? 'inline-flex' : 'none';
        
        // Store initial routing
        this.inputRouting = this.inputRoutingMatrix.getRouting();
      }
      
      // Initialize output routing
      if (this.outputRoutingBtn) {
        // Update or create routing matrix
        if (this.outputRoutingMatrix) {
          this.outputRoutingMatrix.updateChannelCount(outputChannels);
        } else {
          // Add SVG icon to button
          const icon = createRoutingButton();
          this.outputRoutingBtn.innerHTML = '';
          this.outputRoutingBtn.appendChild(icon.querySelector('svg')!);
          
          // Create routing matrix instance
          this.outputRoutingMatrix = new RoutingMatrix(outputChannels);
          this.outputRoutingMatrix.setOnRoutingChange((routing: number[]) => {
            this.outputRouting = routing;
            console.log('Output routing updated:', routing);
          });
          
          // Add click handler to show matrix
          this.outputRoutingBtn.addEventListener('click', () => {
            if (this.outputRoutingMatrix && this.outputRoutingBtn) {
              this.outputRoutingMatrix.show(this.outputRoutingBtn);
            }
          });
        }
        
        // Show button if we have more than 1 channel
        this.outputRoutingBtn.style.display = outputChannels > 1 ? 'inline-flex' : 'none';
        
        // Store initial routing
        this.outputRouting = this.outputRoutingMatrix.getRouting();
      }
      
      console.log(`Routing matrices initialized: ${inputChannels} inputs, ${outputChannels} outputs`);
    } catch (error) {
      console.error('Error initializing routing matrices:', error);
    }
  }
  
  private setDeviceStatus(status: 'success' | 'error' | 'neutral', message?: string, deviceType: 'input' | 'output' = 'input'): void {
    const device = deviceType === 'input' ? this.modalCaptureDevice : this.modalOutputDevice;
    if (!device) return;
    
    // Remove existing status classes
    device.classList.remove('device-success', 'device-error', 'device-neutral');
    
    // Add appropriate class
    if (status === 'success') {
      device.classList.add('device-success');
      console.log(`${deviceType} device status: ✅ Success`, message || '');
    } else if (status === 'error') {
      device.classList.add('device-error');
      console.error(`${deviceType} device status: ❌ Error`, message || '');
    } else {
      device.classList.add('device-neutral');
    }
  }
  
  private populateOutputChannels(channelCount: number): void {
    if (!this.modalOutputChannel) return;
    
    // Save current selection
    const currentValue = this.modalOutputChannel.value;
    
    // Clear existing options
    this.modalOutputChannel.innerHTML = '';
    
    // Add default combined option
    const defaultOption = document.createElement('option');
    defaultOption.value = 'default';
    defaultOption.textContent = 'System Default';
    this.modalOutputChannel.appendChild(defaultOption);
    
    if (channelCount === 1) {
      // Mono device
      const monoOption = document.createElement('option');
      monoOption.value = 'both';
      monoOption.textContent = 'Mono';
      this.modalOutputChannel.appendChild(monoOption);
    } else if (channelCount === 2) {
      // Stereo device - only Left and Right options
      const leftOption = document.createElement('option');
      leftOption.value = 'left';
      leftOption.textContent = 'Left';
      this.modalOutputChannel.appendChild(leftOption);
      
      const rightOption = document.createElement('option');
      rightOption.value = 'right';
      rightOption.textContent = 'Right';
      this.modalOutputChannel.appendChild(rightOption);
    } else if (channelCount === 6) {
      // 5.1 surround
      const allOption = document.createElement('option');
      allOption.value = 'all';
      allOption.textContent = 'All Channels (5.1)';
      this.modalOutputChannel.appendChild(allOption);
      
      const channels = [
        { value: 'ch1', label: 'Front Left' },
        { value: 'ch2', label: 'Front Right' },
        { value: 'ch3', label: 'Center' },
        { value: 'ch4', label: 'LFE (Subwoofer)' },
        { value: 'ch5', label: 'Surround Left' },
        { value: 'ch6', label: 'Surround Right' }
      ];
      
      channels.forEach(ch => {
        const option = document.createElement('option');
        option.value = ch.value;
        option.textContent = ch.label;
        this.modalOutputChannel?.appendChild(option);
      });
    } else if (channelCount === 8) {
      // 7.1 surround
      const allOption = document.createElement('option');
      allOption.value = 'all';
      allOption.textContent = 'All Channels (7.1)';
      this.modalOutputChannel.appendChild(allOption);
      
      const channels = [
        { value: 'ch1', label: 'Front Left' },
        { value: 'ch2', label: 'Front Right' },
        { value: 'ch3', label: 'Center' },
        { value: 'ch4', label: 'LFE (Subwoofer)' },
        { value: 'ch5', label: 'Surround Left' },
        { value: 'ch6', label: 'Surround Right' },
        { value: 'ch7', label: 'Back Left' },
        { value: 'ch8', label: 'Back Right' }
      ];
      
      channels.forEach(ch => {
        const option = document.createElement('option');
        option.value = ch.value;
        option.textContent = ch.label;
        this.modalOutputChannel?.appendChild(option);
      });
    } else if (channelCount === 16) {
      // 9.1.6 Dolby Atmos or similar
      const allOption = document.createElement('option');
      allOption.value = 'all';
      allOption.textContent = 'All Channels (9.1.6 / 16ch)';
      this.modalOutputChannel.appendChild(allOption);
      
      const channels = [
        { value: 'ch1', label: 'Front Left' },
        { value: 'ch2', label: 'Front Right' },
        { value: 'ch3', label: 'Center' },
        { value: 'ch4', label: 'LFE (Subwoofer)' },
        { value: 'ch5', label: 'Surround Left' },
        { value: 'ch6', label: 'Surround Right' },
        { value: 'ch7', label: 'Back Left' },
        { value: 'ch8', label: 'Back Right' },
        { value: 'ch9', label: 'Side Left' },
        { value: 'ch10', label: 'Side Right' },
        { value: 'ch11', label: 'Top Front Left' },
        { value: 'ch12', label: 'Top Front Right' },
        { value: 'ch13', label: 'Top Back Left' },
        { value: 'ch14', label: 'Top Back Right' },
        { value: 'ch15', label: 'Wide Left' },
        { value: 'ch16', label: 'Wide Right' }
      ];
      
      channels.forEach(ch => {
        const option = document.createElement('option');
        option.value = ch.value;
        option.textContent = ch.label;
        this.modalOutputChannel?.appendChild(option);
      });
    } else {
      // Generic multi-channel
      const allOption = document.createElement('option');
      allOption.value = 'all';
      allOption.textContent = `All Channels (${channelCount})`;
      this.modalOutputChannel.appendChild(allOption);
      
      for (let i = 1; i <= channelCount; i++) {
        const option = document.createElement('option');
        option.value = `ch${i}`;
        option.textContent = `Channel ${i}`;
        this.modalOutputChannel.appendChild(option);
      }
    }
    
    // Try to restore previous selection or set default
    if (currentValue && this.modalOutputChannel.querySelector(`option[value="${currentValue}"]`)) {
      this.modalOutputChannel.value = currentValue;
    } else {
      this.modalOutputChannel.value = channelCount === 1 ? 'both' : 'default';
    }
    
    // Update channel display options after changing output channels
    const outputChannel = this.modalOutputChannel.value || 'both';
    this.updateChannelSelectOptions(outputChannel);
    
    console.log(`Populated output channels for ${channelCount}-channel device`);
  }

  private async onCalibrationFileChange(event: Event): Promise<void> {
    const input = event.target as HTMLInputElement;
    const file = input.files?.[0];
    
    if (!file) return;
    
    console.log('Loading calibration file:', file.name);
    
    try {
      const text = await file.text();
      const { frequencies, magnitudes } = this.parseCalibrationFile(text);
      
      if (frequencies.length === 0 || magnitudes.length === 0) {
        throw new Error('No valid data found in calibration file');
      }
      
      if (frequencies.length !== magnitudes.length) {
        throw new Error('Frequency and magnitude arrays have different lengths');
      }
      
      // Set calibration data in graph renderer
      if (this.captureGraphRenderer) {
        this.captureGraphRenderer.setCalibrationData(frequencies, magnitudes);
        
        // Update button states
        if (this.captureCalibrationClear) {
          this.captureCalibrationClear.style.display = 'inline-flex';
        }
        if (this.captureCalibrationBtn) {
          this.captureCalibrationBtn.textContent = '✓ Loaded';
        }
        
        // Re-render current data if available
        if (this.currentCaptureData) {
          this.captureGraphRenderer.renderGraph({
            frequencies: this.currentCaptureData.frequencies,
            rawMagnitudes: this.currentCaptureData.rawMagnitudes,
            smoothedMagnitudes: this.currentCaptureData.smoothedMagnitudes,
            rawPhase: this.currentCaptureData.rawPhase,
            smoothedPhase: this.currentCaptureData.smoothedPhase
          });
        }
        
        console.log(`Calibration loaded: ${frequencies.length} points from ${file.name}`);
      }
    } catch (error) {
      console.error('Error loading calibration file:', error);
      alert(`Failed to load calibration file: ${error instanceof Error ? error.message : 'Unknown error'}`);
      
      // Reset file input
      if (input) {
        input.value = '';
      }
    }
  }
  
  private parseCalibrationFile(text: string): { frequencies: number[], magnitudes: number[] } {
    const frequencies: number[] = [];
    const magnitudes: number[] = [];
    
    const lines = text.split('\n').map(line => line.trim()).filter(line => line.length > 0);
    
    let hasHeader = false;
    
    for (let i = 0; i < lines.length; i++) {
      const line = lines[i];
      
      // Skip comments and header lines
      if (line.startsWith('#') || line.startsWith('//') || 
          (i === 0 && (line.toLowerCase().includes('frequency') || line.toLowerCase().includes('freq')))) {
        if (i === 0) hasHeader = true;
        continue;
      }
      
      // Parse data lines
      const parts = line.split(/[,\t\s]+/).filter(part => part.length > 0);
      
      if (parts.length >= 2) {
        const freq = parseFloat(parts[0]);
        const mag = parseFloat(parts[1]);
        
        if (!isNaN(freq) && !isNaN(mag) && freq > 0) {
          frequencies.push(freq);
          magnitudes.push(mag);
        }
      }
    }
    
    console.log(`Parsed ${frequencies.length} calibration points from file`);
    return { frequencies, magnitudes };
  }
  
  private clearCalibrationFile(): void {
    console.log('Clearing calibration file');
    
    // Clear file input
    if (this.captureCalibrationFile) {
      this.captureCalibrationFile.value = '';
    }
    
    // Clear calibration data in graph renderer
    if (this.captureGraphRenderer) {
      this.captureGraphRenderer.clearCalibrationData();
      
      // Re-render current data if available
      if (this.currentCaptureData) {
        this.captureGraphRenderer.renderGraph({
          frequencies: this.currentCaptureData.frequencies,
          rawMagnitudes: this.currentCaptureData.rawMagnitudes,
          smoothedMagnitudes: this.currentCaptureData.smoothedMagnitudes,
          rawPhase: this.currentCaptureData.rawPhase,
          smoothedPhase: this.currentCaptureData.smoothedPhase,
          channelData: this.currentCaptureData.channelData,
          outputChannel: this.currentCaptureData.outputChannel
        });
      }
    }
    
    // Update button states
    if (this.captureCalibrationClear) {
      this.captureCalibrationClear.style.display = 'none';
    }
    if (this.captureCalibrationBtn) {
      this.captureCalibrationBtn.textContent = '📁 Load File';
    }
  }
  
  private generateChannelData(frequencies: number[], magnitudes: number[], phases: number[], smoothedMagnitudes: number[], smoothedPhase: number[]): any {
    // Generate simulated channel-specific data for demonstration
    // In a real implementation, this would come from actual stereo capture
    
    const leftMagnitudes = magnitudes.map(mag => mag + Math.random() * 2 - 1); // Slight variation
    const rightMagnitudes = magnitudes.map(mag => mag + Math.random() * 2 - 1); // Slight variation
    const averageMagnitudes = magnitudes.map((mag, i) => (leftMagnitudes[i] + rightMagnitudes[i]) / 2);
    
    const leftPhases = phases.map(phase => phase + Math.random() * 10 - 5); // Slight variation
    const rightPhases = phases.map(phase => phase + Math.random() * 10 - 5); // Slight variation
    const averagePhases = phases.map((phase, i) => (leftPhases[i] + rightPhases[i]) / 2);
    
    const leftSmoothedMag = smoothedMagnitudes.map(mag => mag + Math.random() * 1 - 0.5);
    const rightSmoothedMag = smoothedMagnitudes.map(mag => mag + Math.random() * 1 - 0.5);
    const averageSmoothedMag = smoothedMagnitudes.map((mag, i) => (leftSmoothedMag[i] + rightSmoothedMag[i]) / 2);
    
    const leftSmoothedPhase = smoothedPhase.map(phase => phase + Math.random() * 5 - 2.5);
    const rightSmoothedPhase = smoothedPhase.map(phase => phase + Math.random() * 5 - 2.5);
    const averageSmoothedPhase = smoothedPhase.map((phase, i) => (leftSmoothedPhase[i] + rightSmoothedPhase[i]) / 2);
    
    return {
      left: {
        rawMagnitudes: leftMagnitudes,
        smoothedMagnitudes: leftSmoothedMag,
        rawPhase: leftPhases,
        smoothedPhase: leftSmoothedPhase
      },
      right: {
        rawMagnitudes: rightMagnitudes,
        smoothedMagnitudes: rightSmoothedMag,
        rawPhase: rightPhases,
        smoothedPhase: rightSmoothedPhase
      },
      average: {
        rawMagnitudes: averageMagnitudes,
        smoothedMagnitudes: averageSmoothedMag,
        rawPhase: averagePhases,
        smoothedPhase: averageSmoothedPhase
      }
    };
  }

  private setupResizer(): void {
    const resizer = document.getElementById("resizer");
    const leftPanel = document.getElementById("left_panel");

    if (!resizer || !leftPanel) return;

    resizer.addEventListener("mousedown", (e) => {
      this.isResizing = true;
      this.startX = e.clientX;
      this.startWidth = parseInt(
        document.defaultView?.getComputedStyle(leftPanel).width || "0",
        10,
      );
      document.addEventListener("mousemove", this.handleMouseMove);
      document.addEventListener("mouseup", this.handleMouseUp);
      e.preventDefault();
    });
  }

  private handleMouseMove = (e: MouseEvent) => {
    if (!this.isResizing) return;

    const leftPanel = document.getElementById("left_panel");
    if (!leftPanel) return;

    const dx = e.clientX - this.startX;
    const newWidth = this.startWidth + dx;
    const minWidth = 300;
    const maxWidth = window.innerWidth * 0.6;

    if (newWidth >= minWidth && newWidth <= maxWidth) {
      leftPanel.style.width = newWidth + "px";
      // Update CSS custom property for bottom-left to match
      document.documentElement.style.setProperty(
        "--left-panel-width",
        newWidth + "px",
      );
    }
  };

  private handleMouseUp = () => {
    this.isResizing = false;
    document.removeEventListener("mousemove", this.handleMouseMove);
    document.removeEventListener("mouseup", this.handleMouseUp);
  };

  showProgress(show: boolean): void {
    if (this.progressElement) {
      this.progressElement.style.display = show ? "block" : "none";
    }
  }

  updateStatus(message: string): void {
    console.log("Status:", message);
  }

  showError(error: string): void {
    const errorMessageElement = document.getElementById(
      "error_message",
    ) as HTMLElement;
    if (errorMessageElement) {
      errorMessageElement.textContent = error;
    }
    if (this.errorElement) {
      this.errorElement.style.display = "block";
    }
  }

  updateScores(
    before: number | null | undefined,
    after: number | null | undefined,
  ): void {
    const scoreBeforeElement = document.getElementById(
      "score_before",
    ) as HTMLElement;
    const scoreAfterElement = document.getElementById(
      "score_after",
    ) as HTMLElement;
    const scoreImprovementElement = document.getElementById(
      "score_improvement",
    ) as HTMLElement;

    // Handle null/undefined values
    if (scoreBeforeElement) {
      scoreBeforeElement.textContent =
        before !== null && before !== undefined ? before.toFixed(3) : "-";
    }
    if (scoreAfterElement) {
      scoreAfterElement.textContent =
        after !== null && after !== undefined ? after.toFixed(3) : "-";
    }
    if (scoreImprovementElement) {
      if (
        before !== null &&
        before !== undefined &&
        after !== null &&
        after !== undefined
      ) {
        const improvement = after - before;
        scoreImprovementElement.textContent =
          (improvement >= 0 ? "+" : "") + improvement.toFixed(3);
      } else {
        scoreImprovementElement.textContent = "-";
      }
    }

    // Scores are now always visible in the bottom row
  }

  clearResults(): void {
    console.log("clearResults called");
    // Reset scores to default values instead of hiding
    const scoreBeforeElement = document.getElementById(
      "score_before",
    ) as HTMLElement;
    const scoreAfterElement = document.getElementById(
      "score_after",
    ) as HTMLElement;
    const scoreImprovementElement = document.getElementById(
      "score_improvement",
    ) as HTMLElement;

    if (scoreBeforeElement) {
      scoreBeforeElement.textContent = "-";
    }
    if (scoreAfterElement) {
      scoreAfterElement.textContent = "-";
    }
    if (scoreImprovementElement) {
      scoreImprovementElement.textContent = "-";
    }

    if (this.errorElement) {
      this.errorElement.style.display = "none";
    }
  }

  setOptimizationRunning(running: boolean): void {
    if (this.optimizeBtn) {
      this.optimizeBtn.disabled = running;
      this.optimizeBtn.textContent = running
        ? "Optimizing..."
        : "Run Optimization";
    }

    // Update modal buttons based on optimization state
    if (running) {
      this.showCancelButton();
      // Start the timer
      this.optimizationStartTime = Date.now();
      if (this.elapsedTimeElement) {
        this.elapsedTimeElement.textContent = "00:00";
      }
    } else {
      // Reset timer
      this.optimizationStartTime = 0;
    }
  }

  showCancelButton(): void {
    if (this.cancelOptimizationBtn && this.doneOptimizationBtn) {
      this.cancelOptimizationBtn.style.display = "inline-block";
      this.doneOptimizationBtn.style.display = "none";
    }
  }

  showCloseButton(): void {
    if (this.cancelOptimizationBtn && this.doneOptimizationBtn) {
      this.cancelOptimizationBtn.style.display = "none";
      this.doneOptimizationBtn.style.display = "inline-block";

      // Update button text and styling for close functionality
      this.doneOptimizationBtn.textContent = "Close";
      this.doneOptimizationBtn.className = "btn btn-primary"; // Blue button
    }

    // Update progress status to show completion
    if (this.progressStatus) {
      this.progressStatus.textContent = "Optimization Complete";
    }
  }

  openOptimizationModal(): void {
    if (this.optimizationModal) {
      this.optimizationModal.style.display = "flex";
      document.body.style.overflow = "hidden";
    }
  }

  closeOptimizationModal(): void {
    if (this.optimizationModal) {
      this.optimizationModal.style.display = "none";
      document.body.style.overflow = "auto";
    }
  }

  updateProgress(
    stage: string,
    status: string,
    details: string,
    percentage: number,
  ): void {
    console.log(
      `[UI DEBUG] updateProgress called: stage="${stage}", status="${status}", details="${details}"`,
    );

    if (this.progressStatus) {
      this.progressStatus.textContent = `${stage}: ${status}`;
      console.log(
        `[UI DEBUG] Updated progress status text to: "${stage}: ${status}"`,
      );
    } else {
      console.warn("[UI DEBUG] progressStatus element not found!");
    }

    // Update elapsed time
    this.updateElapsedTime();
  }

  private updateElapsedTime(): void {
    if (this.optimizationStartTime > 0 && this.elapsedTimeElement) {
      const elapsedMs = Date.now() - this.optimizationStartTime;
      const elapsedSeconds = Math.floor(elapsedMs / 1000);
      const minutes = Math.floor(elapsedSeconds / 60);
      const seconds = elapsedSeconds % 60;
      const timeString = `${minutes.toString().padStart(2, "0")}:${seconds.toString().padStart(2, "0")}`;

      this.elapsedTimeElement.textContent = timeString;
      console.log(`[UI DEBUG] Updated elapsed time to: ${timeString}`);
    }
  }

  // toggleAccordion method removed - using grid layout

  collapseAllAccordion(): void {
    // Grid layout - accordion functionality removed
    console.log(
      "[UI] Accordion collapse functionality not needed in grid layout",
    );
  }

  showAccordionSection(sectionId: string): void {
    // Grid layout - accordion functionality removed
    console.log(
      `[UI] Grid layout - section ${sectionId} visibility managed automatically`,
    );
  }

  setEQEnabled(enabled: boolean): void {
    this.eqEnabled = enabled;

    // Update button states
    if (this.eqOnBtn && this.eqOffBtn) {
      if (enabled) {
        this.eqOnBtn.classList.add("active");
        this.eqOffBtn.classList.remove("active");
      } else {
        this.eqOnBtn.classList.remove("active");
        this.eqOffBtn.classList.add("active");
      }
    }

    console.log(`EQ ${enabled ? "enabled" : "disabled"}`);
  }

  resetToDefaults(): void {
    // Reset form to default values
    const form = this.form;
    if (form) {
      form.reset();

      // Set specific default values with null checks
      const setElementValue = (
        id: string,
        value: string | number | boolean,
        optional: boolean = false,
      ) => {
        const element = document.getElementById(id) as
          | HTMLInputElement
          | HTMLSelectElement;
        if (element) {
          if (element.type === "checkbox") {
            (element as HTMLInputElement).checked = Boolean(value);
          } else {
            element.value = String(value);
          }
          console.log(`Set ${id} = ${value}`);
        } else if (!optional) {
          console.warn(`Element with id '${id}' not found`);
        }
      };

      // Set input source radio button
      const inputSourceRadio = document.querySelector(
        `input[name="input_source"][value="${OPTIMIZATION_DEFAULTS.input_source}"]`,
      ) as HTMLInputElement;
      if (inputSourceRadio) {
        inputSourceRadio.checked = true;
      }

      // Core EQ parameters
      setElementValue("num_filters", OPTIMIZATION_DEFAULTS.num_filters);
      setElementValue("sample_rate", OPTIMIZATION_DEFAULTS.sample_rate);
      setElementValue("min_db", OPTIMIZATION_DEFAULTS.min_db);
      setElementValue("max_db", OPTIMIZATION_DEFAULTS.max_db);
      setElementValue("min_q", OPTIMIZATION_DEFAULTS.min_q);
      setElementValue("max_q", OPTIMIZATION_DEFAULTS.max_q);
      setElementValue("min_freq", OPTIMIZATION_DEFAULTS.min_freq);
      setElementValue("max_freq", OPTIMIZATION_DEFAULTS.max_freq);
      setElementValue("curve_name", OPTIMIZATION_DEFAULTS.curve_name);
      setElementValue("loss", OPTIMIZATION_DEFAULTS.loss);
      setElementValue("peq_model", "pk"); // Default PEQ model

      // Algorithm parameters
      setElementValue("algo", OPTIMIZATION_DEFAULTS.algo);
      setElementValue("population", OPTIMIZATION_DEFAULTS.population);
      setElementValue("maxeval", OPTIMIZATION_DEFAULTS.maxeval);
      setElementValue("strategy", OPTIMIZATION_DEFAULTS.strategy, true);
      setElementValue("de_f", OPTIMIZATION_DEFAULTS.de_f, true);
      setElementValue("de_cr", OPTIMIZATION_DEFAULTS.de_cr, true);
      setElementValue(
        "adaptive_weight_f",
        OPTIMIZATION_DEFAULTS.adaptive_weight_f,
        true,
      );
      setElementValue(
        "adaptive_weight_cr",
        OPTIMIZATION_DEFAULTS.adaptive_weight_cr,
        true,
      );

      // Spacing parameters
      setElementValue("min_spacing_oct", OPTIMIZATION_DEFAULTS.min_spacing_oct);
      setElementValue("spacing_weight", OPTIMIZATION_DEFAULTS.spacing_weight);

      // Tolerance parameters
      setElementValue("tolerance", OPTIMIZATION_DEFAULTS.tolerance);
      setElementValue("abs_tolerance", OPTIMIZATION_DEFAULTS.abs_tolerance);

      // Refinement parameters
      setElementValue("refine", OPTIMIZATION_DEFAULTS.refine);
      setElementValue("local_algo", OPTIMIZATION_DEFAULTS.local_algo, true);

      // Smoothing parameters
      setElementValue("smooth", OPTIMIZATION_DEFAULTS.smooth);
      setElementValue("smooth_n", OPTIMIZATION_DEFAULTS.smooth_n);
    }

    this.updateConditionalParameters();
    console.log("Form reset to defaults");
  }

  updateConditionalParameters(): void {
    const algo = (document.getElementById("algo") as HTMLSelectElement)?.value;
    const inputType = (
      document.querySelector(
        'input[name="input_source"]:checked',
      ) as HTMLInputElement
    )?.value;

    // Show/hide DE-specific parameters
    const deParams = document.getElementById("de-params");
    if (deParams) {
      deParams.style.display = algo === "autoeq_de" ? "block" : "none";
    }

    // Show/hide speaker selection
    const speakerSelection = document.getElementById("speaker-selection");
    if (speakerSelection) {
      speakerSelection.style.display =
        inputType === "speaker" ? "block" : "none";
    }

    // Show/hide file selection
    const fileSelection = document.getElementById("file-selection");
    if (fileSelection) {
      fileSelection.style.display = inputType === "file" ? "block" : "none";
    }

    // Show/hide capture section
    const captureSection = document.getElementById("capture-section");
    if (captureSection) {
      captureSection.style.display = inputType === "capture" ? "block" : "none";
    }

    // Show/hide curve selection based on input type
    const curveNameParam = document
      .getElementById("curve_name")
      ?.closest(".param-item") as HTMLElement;
    if (curveNameParam) {
      // Hide curve selection for headphones (they use targets instead)
      curveNameParam.style.display =
        inputType === "headphone" ? "none" : "block";
    }

    // Update loss function options based on input type
    const lossSelect = document.getElementById("loss") as HTMLSelectElement;
    if (lossSelect) {
      this.updateLossOptions(inputType, lossSelect);
    }
  }

  private switchTab(tabName: string): void {
    console.log("Switching to tab:", tabName);

    // Remove active class from all tab labels
    const tabLabels = document.querySelectorAll(".tab-label");
    tabLabels.forEach((label) => label.classList.remove("active"));

    // Add active class to current tab label
    const activeTabLabel = document.querySelector(
      `.tab-label[data-tab="${tabName}"]`,
    );
    if (activeTabLabel) {
      activeTabLabel.classList.add("active");
    }

    // Hide all tab content
    const tabContents = document.querySelectorAll(".tab-content");
    tabContents.forEach((content) => content.classList.remove("active"));

    // Show current tab content
    const activeTabContent = document.getElementById(`${tabName}_inputs`);
    if (activeTabContent) {
      activeTabContent.classList.add("active");
    } else {
      console.warn(`Tab content for '${tabName}' not found`);
    }

    // Set appropriate loss function based on tab
    const lossSelect = document.getElementById("loss") as HTMLSelectElement;
    if (lossSelect) {
      if (tabName === "speaker") {
        // Only set to speaker-flat if current value is not a speaker option
        if (!lossSelect.value.startsWith("speaker-")) {
          lossSelect.value = "speaker-flat";
          console.log("Set loss function to speaker-flat for speaker tab");
        }
      } else if (tabName === "headphone") {
        // Only set to headphone-flat if current value is not a headphone option
        if (!lossSelect.value.startsWith("headphone-")) {
          lossSelect.value = "headphone-flat";
          console.log("Set loss function to headphone-flat for headphone tab");
        }
      }
      // For 'file' and 'capture' tabs, keep whatever value is currently selected
    }
  }

  private updateLossOptions(
    inputType: string,
    lossSelect: HTMLSelectElement,
  ): void {
    // Import loss options
    import("./optimization-constants").then(
      ({ LOSS_OPTIONS, SPEAKER_LOSS_OPTIONS, HEADPHONE_LOSS_OPTIONS }) => {
        const currentValue = lossSelect.value;

        // Clear existing options
        lossSelect.innerHTML = "";

        // Determine which options to use
        let options;
        let defaultValue;

        if (inputType === "headphone") {
          // Headphone: only headphone options
          options = HEADPHONE_LOSS_OPTIONS;
          defaultValue = "headphone-flat";
        } else if (inputType === "speaker") {
          // Speaker: only speaker options
          options = SPEAKER_LOSS_OPTIONS;
          defaultValue = "speaker-flat";
        } else {
          // File, Capture, or any other: show all 4 options
          options = LOSS_OPTIONS;
          defaultValue = "speaker-flat";
        }

        // Populate with appropriate options
        Object.entries(options).forEach(([value, label]) => {
          const option = document.createElement("option");
          option.value = value;
          option.textContent = label;
          lossSelect.appendChild(option);
        });

        // Try to keep the current value if it's still valid, otherwise set default
        if (lossSelect.querySelector(`option[value="${currentValue}"]`)) {
          lossSelect.value = currentValue;
        } else {
          lossSelect.value = defaultValue;
        }
      },
    );
  }

  // Event handlers (to be connected to main application logic)
  private onOptimizeClick(): void {
    // This will be connected to the main optimization logic
    console.log("Optimize button clicked");
  }

  private async onCaptureClick(): Promise<void> {
    console.log("Capture button clicked");

    if (!this.captureBtn) return;

    const isCapturing = this.captureBtn.textContent?.includes("Stop");

    if (isCapturing) {
      // Stop capture
      this.stopCapture();
    } else {
      // Start capture
      await this.startCapture();
    }
  }

  private async startCapture(): Promise<void> {
    if (!this.captureBtn || !this.captureStatus || !this.captureStatusText)
      return;

    try {
      // Update UI to capturing state
      this.captureBtn.textContent = "⏹️ Stop Capture";
      this.captureBtn.classList.add("capturing");
      this.captureStatus.style.display = "block";
      this.captureStatusText.textContent = "Starting capture...";

      // Hide any previous results
      if (this.captureResult) {
        this.captureResult.style.display = "none";
      }

      // Import AudioProcessor dynamically to avoid circular dependencies
      const { AudioProcessor } = await import("./audio/audio-processor");
      const audioProcessor = new AudioProcessor();

      try {
        // First enumerate and populate audio devices if needed
        if (
          this.captureDeviceSelect &&
          this.captureDeviceSelect.options.length <= 1
        ) {
          await this.populateAudioDevices(audioProcessor);
        }

        // Get selected device
        const selectedDevice = this.captureDeviceSelect?.value || "default";

        // Set sweep duration if selected
        if (this.sweepDurationSelect) {
          const duration = parseInt(this.sweepDurationSelect.value) || 10;
          audioProcessor.setSweepDuration(duration);
        }

        // Set output channel if selected
        if (this.outputChannelSelect) {
          const channel = this.outputChannelSelect.value as
            | "left"
            | "right"
            | "both"
            | "default";
          audioProcessor.setOutputChannel(channel);
          console.log("Setting output channel to:", channel);
        }

        // Set sample rate if selected
        if (this.captureSampleRateSelect) {
          const sampleRate =
            parseInt(this.captureSampleRateSelect.value) || 48000;
          audioProcessor.setSampleRate(sampleRate);
          console.log("Setting sample rate to:", sampleRate);
        }

        // Set signal type if selected
        if (this.signalTypeSelect) {
          const signalType = this.signalTypeSelect.value as
            | "sweep"
            | "white"
            | "pink";
          audioProcessor.setSignalType(signalType);
          console.log("Setting signal type to:", signalType);
        }

        const signalType = this.signalTypeSelect?.value || "sweep";
        this.captureStatusText.textContent = `Playing ${signalType === "sweep" ? "frequency sweep" : signalType + " noise"} and capturing response...`;

        // Start the capture with selected device
        const result = await audioProcessor.startCapture(selectedDevice);

        if (result.success && result.frequencies.length > 0) {
          console.log(
            "Capture successful:",
            result.frequencies.length,
            "points",
          );

          // Call the callback to store captured data in the optimization manager
          if (this.onCaptureComplete) {
            this.onCaptureComplete(result.frequencies, result.magnitudes);
          }

          this.captureStatusText.textContent = `✅ Captured ${result.frequencies.length} frequency points`;

          // Show results
          if (this.captureResult) {
            this.captureResult.style.display = "block";
          }

          // Plot the captured data
          this.plotCapturedData(result.frequencies, result.magnitudes);
        } else {
          throw new Error(result.error || "Capture failed");
        }
      } finally {
        audioProcessor.destroy();
      }
    } catch (error) {
      console.error("Capture error:", error);

      if (this.captureStatusText) {
        this.captureStatusText.textContent = `❌ Capture failed: ${error instanceof Error ? error.message : "Unknown error"}`;
      }
    } finally {
      // Reset UI
      if (this.captureBtn) {
        this.captureBtn.textContent = "🎤 Start Capture";
        this.captureBtn.classList.remove("capturing");
      }
    }
  }

  private stopCapture(): void {
    console.log("Stopping capture...");

    // Reset UI immediately
    if (this.captureBtn) {
      this.captureBtn.textContent = "🎤 Start Capture";
      this.captureBtn.classList.remove("capturing");
    }

    if (this.captureStatusText) {
      this.captureStatusText.textContent = "Capture stopped";
    }
  }

  private async populateAudioDevices(audioProcessor: any): Promise<void> {
    if (!this.captureDeviceSelect) return;

    try {
      const devices = await audioProcessor.enumerateAudioDevices();

      // Clear existing options
      this.captureDeviceSelect.innerHTML = "";

      // Add default option
      const defaultOption = document.createElement("option");
      defaultOption.value = "default";
      defaultOption.textContent = "Default Microphone";
      this.captureDeviceSelect.appendChild(defaultOption);

      // Add all available devices
      devices.forEach((device: MediaDeviceInfo) => {
        const option = document.createElement("option");
        option.value = device.deviceId;
        option.textContent =
          device.label || `Microphone ${device.deviceId.substr(0, 8)}`;
        this.captureDeviceSelect?.appendChild(option);
      });

      console.log(`Populated ${devices.length} audio input devices`);
    } catch (error) {
      console.error("Error populating audio devices:", error);
    }
  }

  private plotCapturedData(frequencies: number[], magnitudes: number[]): void {
    console.log("Plotting captured data...");

    if (!this.capturePlot) {
      console.warn("Capture plot element not found");
      return;
    }

    // Clear existing content
    this.capturePlot.innerHTML = "";

    // Create a canvas for the plot
    const canvas = document.createElement("canvas");
    canvas.width = this.capturePlot.offsetWidth || 600;
    canvas.height = 250;
    canvas.style.width = "100%";
    canvas.style.height = "250px";
    canvas.style.backgroundColor = "#f8f9fa";
    canvas.style.border = "1px solid #dee2e6";
    canvas.style.borderRadius = "4px";

    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    // Add canvas to plot container
    this.capturePlot.appendChild(canvas);

    // Calculate plot dimensions
    const padding = 50;
    const plotWidth = canvas.width - 2 * padding;
    const plotHeight = canvas.height - 2 * padding;

    // Find min/max values for scaling
    const minMag = Math.min(...magnitudes);
    const maxMag = Math.max(...magnitudes);
    const magRange = maxMag - minMag || 1;

    // Draw background
    ctx.fillStyle = "#ffffff";
    ctx.fillRect(padding, padding, plotWidth, plotHeight);

    // Draw grid lines
    ctx.strokeStyle = "#e0e0e0";
    ctx.lineWidth = 0.5;

    // Horizontal grid lines
    for (let i = 0; i <= 5; i++) {
      const y = padding + (i * plotHeight) / 5;
      ctx.beginPath();
      ctx.moveTo(padding, y);
      ctx.lineTo(padding + plotWidth, y);
      ctx.stroke();
    }

    // Vertical grid lines (logarithmic)
    const freqPoints = [20, 50, 100, 200, 500, 1000, 2000, 5000, 10000, 20000];
    freqPoints.forEach((freq) => {
      const x =
        padding + (Math.log10(freq / 20) / Math.log10(1000)) * plotWidth;
      ctx.beginPath();
      ctx.moveTo(x, padding);
      ctx.lineTo(x, padding + plotHeight);
      ctx.stroke();
    });

    // Draw axes
    ctx.strokeStyle = "#333";
    ctx.lineWidth = 2;
    ctx.beginPath();
    ctx.moveTo(padding, padding);
    ctx.lineTo(padding, padding + plotHeight);
    ctx.lineTo(padding + plotWidth, padding + plotHeight);
    ctx.stroke();

    // Draw frequency response curve
    ctx.strokeStyle = "#007bff";
    ctx.lineWidth = 2;
    ctx.beginPath();

    for (let i = 0; i < frequencies.length; i++) {
      const x =
        padding +
        (Math.log10(frequencies[i] / 20) / Math.log10(1000)) * plotWidth;
      const y =
        padding +
        plotHeight -
        ((magnitudes[i] - minMag) / magRange) * plotHeight;

      if (i === 0) {
        ctx.moveTo(x, y);
      } else {
        ctx.lineTo(x, y);
      }
    }

    ctx.stroke();

    // Draw labels
    ctx.fillStyle = "#333";
    ctx.font = "12px sans-serif";
    ctx.textAlign = "center";

    // X-axis labels
    freqPoints.forEach((freq) => {
      const x =
        padding + (Math.log10(freq / 20) / Math.log10(1000)) * plotWidth;
      ctx.fillText(
        freq >= 1000 ? `${freq / 1000}k` : `${freq}`,
        x,
        canvas.height - 25,
      );
    });

    // Y-axis labels
    ctx.textAlign = "right";
    for (let i = 0; i <= 5; i++) {
      const mag = minMag + (1 - i / 5) * magRange;
      const y = padding + (i * plotHeight) / 5;
      ctx.fillText(`${mag.toFixed(1)} dB`, padding - 5, y + 4);
    }

    // Title
    ctx.textAlign = "center";
    ctx.font = "bold 14px sans-serif";
    ctx.fillText("Captured Frequency Response", canvas.width / 2, 20);

    // Axis labels
    ctx.font = "12px sans-serif";
    ctx.fillText("Frequency (Hz)", canvas.width / 2, canvas.height - 5);

    // Rotate for Y-axis label
    ctx.save();
    ctx.translate(15, canvas.height / 2);
    ctx.rotate(-Math.PI / 2);
    ctx.fillText("Magnitude (dB)", 0, 0);
    ctx.restore();

    console.log("Capture plot rendered successfully");
  }

  private async initializeAudioDevices(): Promise<void> {
    // Populate audio devices on initialization
    if (this.captureDeviceSelect) {
      try {
        const { AudioProcessor } = await import("./audio/audio-processor");
        const audioProcessor = new AudioProcessor();
        await this.populateAudioDevices(audioProcessor);
        audioProcessor.destroy();
      } catch (error) {
        console.error("Error initializing audio devices:", error);
      }
    }
  }

  private clearCaptureResults(): void {
    // Clear the UI
    if (this.captureResult) {
      this.captureResult.style.display = "none";
    }
    if (this.capturePlot) {
      this.capturePlot.innerHTML = "";
    }
    if (this.captureStatusText) {
      this.captureStatusText.textContent = "Ready to capture";
    }

    // Clear stored data by notifying with empty arrays
    if (this.onCaptureComplete) {
      this.onCaptureComplete([], []);
    }

    console.log("Capture results cleared");
  }

  private onListenClick(): void {
    // This will be connected to the audio logic
    console.log("TODO: Listen button clicked");
  }

  private onStopClick(): void {
    // This will be connected to the audio logic
    console.log("TODO: Stop button clicked");
  }

  private cancelOptimization(): void {
    // This will be connected to the optimization logic
    console.log("TODO: Cancel optimization");
  }

  // Capture Modal Management Methods
  private async openCaptureModal(): Promise<void> {
    console.log("Opening capture modal...");
    
    if (!this.captureModal) {
      console.error("Capture modal not found");
      return;
    }

    // Show modal
    this.captureModal.style.display = "flex";
    document.body.style.overflow = "hidden";

    // Initialize modal state
    await this.initializeCaptureModal();
  }

  private closeCaptureModal(): void {
    console.log("Closing capture modal...");
    
    if (!this.captureModal) return;

    // Stop any ongoing capture
    this.stopModalCapture();

    // Hide modal
    this.captureModal.style.display = "none";
    document.body.style.overflow = "auto";

    // Clean up graph renderer
    if (this.captureGraphRenderer) {
      this.captureGraphRenderer.destroy();
      this.captureGraphRenderer = null;
    }
  }

  private async initializeCaptureModal(): Promise<void> {
    console.log("Initializing capture modal...");
    
    // Initialize device status to neutral
    this.setDeviceStatus('neutral');
    
    // Initialize graph renderer
    if (this.captureModalGraph) {
      try {
        const { CaptureGraphRenderer } = await import("./audio/capture-graph");
        this.captureGraphRenderer = new CaptureGraphRenderer(this.captureModalGraph);
        this.captureGraphRenderer.renderPlaceholder();
        
        // Expose renderer for debugging
        (window as any).debugCaptureGraphRenderer = this.captureGraphRenderer;
      } catch (error) {
        console.error("Error initializing capture graph:", error);
      }
    }

    // Show placeholder, hide graph and progress
    if (this.captureModalPlaceholder) {
      this.captureModalPlaceholder.style.display = "flex";
    }
    if (this.captureModalProgress) {
      this.captureModalProgress.style.display = "none";
    }

    // Reset button states
    this.resetModalButtons();

    // Populate audio devices if needed
    await this.populateModalAudioDevices();
    
    // Update sample rate and bit depth for current device
    await this.updateSampleRateForDevice();
    
    // Update output channel options based on device capabilities
    // This will also update the device status to success or error
    await this.updateOutputChannelOptions();
    
    // Initialize volume sliders appearance
    this.onVolumeChange();
    this.onOutputVolumeChange();
    
    // Render records list in sidebar
    await this.renderRecordsList();
  }

  private resetModalButtons(): void {
    if (this.captureModalStart) {
      this.captureModalStart.style.display = "inline-flex";
      this.captureModalStart.disabled = false;
    }
    if (this.captureModalStop) {
      this.captureModalStop.style.display = "none";
    }
    if (this.captureModalExport) {
      this.captureModalExport.style.display = "none";
    }
  }

  private async populateModalAudioDevices(): Promise<void> {
    if (!this.modalCaptureDevice || !this.modalOutputDevice) return;

    try {
      // Use the new enhanced device manager
      const { AudioDeviceManager } = await import("./audio/device-manager");
      const deviceManager = new AudioDeviceManager(true); // Prefer cpal devices
      
      // Enumerate all devices from both cpal and WebAudio
      const devices = await deviceManager.enumerateDevices();
      
      // Clear existing input options
      this.modalCaptureDevice.innerHTML = "";

      // Add default input option
      const defaultInputOption = document.createElement("option");
      defaultInputOption.value = "default";
      defaultInputOption.textContent = "System Default";
      this.modalCaptureDevice.appendChild(defaultInputOption);

      // Add all available input devices
      const inputList = deviceManager.getDeviceList('input');
      inputList.forEach(device => {
        const option = document.createElement("option");
        option.value = device.value;
        option.textContent = device.label;
        if (device.info) {
          option.title = device.info; // Show extra info as tooltip
        }
        this.modalCaptureDevice?.appendChild(option);
      });

      console.log(`Populated ${devices.input.length} input devices (cpal + WebAudio)`);
      
      // Clear existing output options
      this.modalOutputDevice.innerHTML = "";
      
      // Add default output option
      const defaultOutputOption = document.createElement("option");
      defaultOutputOption.value = "default";
      defaultOutputOption.textContent = "System Default";
      this.modalOutputDevice.appendChild(defaultOutputOption);
      
      // Add all available output devices
      const outputList = deviceManager.getDeviceList('output');
      outputList.forEach(device => {
        const option = document.createElement("option");
        option.value = device.value;
        option.textContent = device.label;
        if (device.info) {
          option.title = device.info; // Show extra info as tooltip
        }
        this.modalOutputDevice?.appendChild(option);
      });
      
      console.log(`Populated ${devices.output.length} output devices (cpal + WebAudio)`);
      
      // Store device manager for later use
      (this as any).deviceManager = deviceManager;
    } catch (error) {
      console.error("Error populating modal audio devices:", error);
    }
  }

  private async startModalCapture(): Promise<void> {
    console.log("Starting modal capture...");
    
    if (!this.captureModalStart || !this.captureModalStop) return;

    try {
      // Update button states
      this.captureModalStart.style.display = "none";
      this.captureModalStop.style.display = "inline-flex";

      // Show progress, hide placeholder
      if (this.captureModalProgress) {
        this.captureModalProgress.style.display = "block";
      }
      if (this.captureModalPlaceholder) {
        this.captureModalPlaceholder.style.display = "none";
      }

      // Update status
      if (this.captureModalStatus) {
        this.captureModalStatus.textContent = "Starting capture...";
      }

      // Get capture parameters from modal controls
      const selectedDevice = this.modalCaptureDevice?.value || "default";
      const selectedOutputDevice = this.modalOutputDevice?.value || "default";
      const outputChannel = (this.modalOutputChannel?.value as "left" | "right" | "both" | "default") || "both";
      const signalType = (this.modalSignalType?.value as "sweep" | "white" | "pink") || "sweep";
      const duration = parseInt(this.modalSweepDuration?.value || "10");
      // Parse sample rate from badge text (e.g., "48kHz" -> 48000)
      const sampleRateText = this.modalCaptureSampleRate?.textContent || "48kHz";
      const sampleRate = sampleRateText.includes("kHz") 
        ? parseFloat(sampleRateText.replace("kHz", "")) * 1000 
        : parseInt(sampleRateText.replace("Hz", ""));

      // Import and setup audio processor
      const { AudioProcessor } = await import("./audio/audio-processor");
      const audioProcessor = new AudioProcessor();
      
      // Configure audio processor
      audioProcessor.setSweepDuration(duration);
      audioProcessor.setOutputChannel(outputChannel);
      audioProcessor.setSampleRate(sampleRate);
      audioProcessor.setSignalType(signalType);
      audioProcessor.setCaptureVolume(this.captureVolume);
      audioProcessor.setOutputVolume(this.outputVolume);
      audioProcessor.setOutputDevice(selectedOutputDevice);

      console.log("Capture parameters:", {
        inputDevice: selectedDevice,
        outputDevice: selectedOutputDevice,
        outputChannel,
        signalType,
        duration,
        sampleRate,
        inputVolume: this.captureVolume,
        outputVolume: this.outputVolume
      });

      // Update status message
      if (this.captureModalStatus) {
        const signalName = signalType === "sweep" ? "frequency sweep" : `${signalType} noise`;
        this.captureModalStatus.textContent = `Playing ${signalName} and capturing response...`;
      }

      // Start capture
      const result = await audioProcessor.startCapture(selectedDevice);
      
      if (result.success && result.frequencies.length > 0) {
        await this.handleSuccessfulCapture(result, {
          device: selectedDevice,
          outputChannel,
          signalType,
          duration,
          sampleRate
        });
      } else {
        throw new Error(result.error || "Capture failed");
      }

      audioProcessor.destroy();
    } catch (error) {
      console.error("Modal capture error:", error);
      this.handleCaptureError(error);
    }
  }

  private async handleSuccessfulCapture(result: any, params: any): Promise<void> {
    console.log("Processing successful capture with phase data...");
    
    // Generate smoothed data using selected smoothing setting
    const { CaptureGraphRenderer } = await import("./audio/capture-graph");
    const octaveFraction = this.captureSmoothingSelect ? 
      parseInt(this.captureSmoothingSelect.value) : 3;
      
    const smoothedMagnitudes = CaptureGraphRenderer.applySmoothing(
      result.frequencies,
      result.magnitudes,
      octaveFraction
    );

    // Generate smoothed phase data if available
    let smoothedPhase: number[] = [];
    if (result.phases && result.phases.length > 0) {
      smoothedPhase = CaptureGraphRenderer.applyPhaseSmoothing(
        result.frequencies,
        result.phases,
        octaveFraction
      );
    }

    // Generate channel-specific data for demonstration
    const channelData = this.generateChannelData(result.frequencies, result.magnitudes, result.phases || [], smoothedMagnitudes, smoothedPhase);

    // Store capture data
    this.currentCaptureData = {
      frequencies: result.frequencies,
      rawMagnitudes: result.magnitudes,
      smoothedMagnitudes,
      rawPhase: result.phases || [],
      smoothedPhase,
      channelData,
      outputChannel: params.outputChannel,
      metadata: {
        timestamp: new Date(),
        deviceName: params.device === "default" ? "Default Microphone" : "Selected Device",
        signalType: params.signalType,
        duration: params.duration,
        sampleRate: params.sampleRate,
        outputChannel: params.outputChannel
      }
    };

    // Update channel options based on actual output channel
    this.updateChannelSelectOptions(params.outputChannel);
    
    // Update graph
    if (this.captureGraphRenderer) {
      this.captureGraphRenderer.renderGraph({
        frequencies: result.frequencies,
        rawMagnitudes: result.magnitudes,
        smoothedMagnitudes,
        rawPhase: result.phases,
        smoothedPhase,
        channelData,
        outputChannel: params.outputChannel
      });
    }

    // Update status
    if (this.captureModalStatus) {
      this.captureModalStatus.textContent = `✅ Captured ${result.frequencies.length} frequency points`;
    }

    // Update button states
    this.captureModalStart!.style.display = "inline-flex";
    this.captureModalStop!.style.display = "none";
    this.captureModalExport!.style.display = "inline-flex";

    // Progress bar to 100%
    if (this.captureModalProgressFill) {
      this.captureModalProgressFill.style.width = "100%";
    }

    // Save to storage
    try {
      const { CaptureStorage } = await import("./audio/capture-storage");
      const captureId = CaptureStorage.saveCapture({
        timestamp: this.currentCaptureData.metadata.timestamp,
        deviceName: this.currentCaptureData.metadata.deviceName,
        signalType: this.currentCaptureData.metadata.signalType,
        duration: this.currentCaptureData.metadata.duration,
        sampleRate: this.currentCaptureData.metadata.sampleRate,
        outputChannel: this.currentCaptureData.metadata.outputChannel,
        frequencies: this.currentCaptureData.frequencies,
        rawMagnitudes: this.currentCaptureData.rawMagnitudes,
        smoothedMagnitudes: this.currentCaptureData.smoothedMagnitudes,
        rawPhase: this.currentCaptureData.rawPhase,
        smoothedPhase: this.currentCaptureData.smoothedPhase
      });
      console.log("Capture saved to storage with ID:", captureId);
      
      // Refresh the records list
      await this.renderRecordsList();
    } catch (error) {
      console.error("Failed to save capture to storage:", error);
    }

    // Call the callback to store captured data in the optimization manager
    if (this.onCaptureComplete) {
      this.onCaptureComplete(result.frequencies, smoothedMagnitudes);
    }

    console.log("Modal capture completed successfully");
  }

  private handleCaptureError(error: any): void {
    console.error("Capture failed:", error);
    
    const errorMessage = error instanceof Error ? error.message : "Unknown error";
    
    // Update status with detailed error message and instructions
    if (this.captureModalStatus) {
      let statusHTML = `<div class="capture-error"><strong>❌ Capture Failed</strong><br><span class="error-message">${errorMessage}</span>`;
      
      // Add specific instructions based on error type
      if (errorMessage.includes("permission denied")) {
        statusHTML += '<br><br><div class="error-instructions">📝 <strong>To fix this:</strong><br>1. Click the microphone icon in your browser\'s address bar<br>2. Select "Always allow" for microphone access<br>3. Refresh the page and try again</div>';
      } else if (errorMessage.includes("No microphone found")) {
        statusHTML += '<br><br><div class="error-instructions">📝 <strong>To fix this:</strong><br>1. Connect a microphone to your device<br>2. Check your system audio settings<br>3. Try a different input device from the dropdown</div>';
      } else if (errorMessage.includes("already in use")) {
        statusHTML += '<br><br><div class="error-instructions">📝 <strong>To fix this:</strong><br>1. Close other applications using your microphone<br>2. Check for running video calls or recording software<br>3. Try again in a few moments</div>';
      } else if (errorMessage.includes("not supported")) {
        statusHTML += '<br><br><div class="error-instructions">📝 <strong>To fix this:</strong><br>1. Use a modern browser (Chrome, Firefox, Edge)<br>2. Make sure you\'re using HTTPS or localhost<br>3. Check that your browser supports WebRTC</div>';
      } else {
        statusHTML += '<br><br><div class="error-instructions">📝 <strong>Try these steps:</strong><br>1. Refresh the page<br>2. Check your microphone connection<br>3. Try a different browser<br>4. Ensure microphone permissions are granted</div>';
      }
      
      statusHTML += '</div>';
      this.captureModalStatus.innerHTML = statusHTML;
    }

    // Reset button states - show Start button for retry
    this.resetModalButtons();
    
    // Make sure start button is enabled for retry
    if (this.captureModalStart) {
      this.captureModalStart.disabled = false;
      this.captureModalStart.textContent = "Retry Capture";
    }

    // Hide progress
    if (this.captureModalProgress) {
      this.captureModalProgress.style.display = "none";
    }
    if (this.captureModalPlaceholder) {
      this.captureModalPlaceholder.style.display = "flex";
    }
  }

  private stopModalCapture(): void {
    console.log("Stopping modal capture...");
    
    // Reset button states
    this.resetModalButtons();

    // Update status
    if (this.captureModalStatus) {
      this.captureModalStatus.textContent = "Capture stopped";
    }
  }

  private exportCaptureCSV(): void {
    console.log("Exporting capture data to CSV...");
    
    if (!this.currentCaptureData) {
      console.warn("No capture data available for export");
      return;
    }

    // Import CSV exporter dynamically
    import("./audio/csv-export").then(({ CSVExporter }) => {
      const exportData = {
        frequencies: this.currentCaptureData!.frequencies,
        rawMagnitudes: this.currentCaptureData!.rawMagnitudes,
        smoothedMagnitudes: this.currentCaptureData!.smoothedMagnitudes,
        rawPhase: this.currentCaptureData!.rawPhase,
        smoothedPhase: this.currentCaptureData!.smoothedPhase,
        metadata: this.currentCaptureData!.metadata
      };

      // Validate data
      const errors = CSVExporter.validateExportData(exportData);
      if (errors.length > 0) {
        console.error("Export validation errors:", errors);
        alert("Cannot export data: " + errors.join(", "));
        return;
      }

      // Export to CSV
      CSVExporter.exportToCSV(exportData);
      console.log("CSV export completed");
    }).catch((error) => {
      console.error("Error importing CSV exporter:", error);
      alert("Failed to export CSV: " + error.message);
    });
  }

  // Set callbacks for external interactions
  setCaptureCompleteCallback(
    callback: (frequencies: number[], magnitudes: number[]) => void,
  ): void {
    this.onCaptureComplete = callback;
  }
  
  setOutputDeviceChangeCallback(
    callback: (deviceId: string) => void,
  ): void {
    this.outputDeviceChangeCallback = callback;
  }

  // Getters for accessing UI elements from main application
  getForm(): HTMLFormElement {
    return this.form;
  }
  getOptimizeBtn(): HTMLButtonElement {
    return this.optimizeBtn;
  }
  getResetBtn(): HTMLButtonElement {
    return this.resetBtn;
  }
  getListenBtn(): HTMLButtonElement {
    return this.listenBtn;
  }
  getStopBtn(): HTMLButtonElement {
    return this.stopBtn;
  }
  getEqOnBtn(): HTMLButtonElement {
    return this.eqOnBtn;
  }
  getEqOffBtn(): HTMLButtonElement {
    return this.eqOffBtn;
  }
  getCancelOptimizationBtn(): HTMLButtonElement {
    return this.cancelOptimizationBtn;
  }

  updateOptimizeBtn(btn: HTMLButtonElement): void {
    this.optimizeBtn = btn;
  }
  updateResetBtn(btn: HTMLButtonElement): void {
    this.resetBtn = btn;
  }
  updateListenBtn(btn: HTMLButtonElement): void {
    this.listenBtn = btn;
  }
  updateStopBtn(btn: HTMLButtonElement): void {
    this.stopBtn = btn;
  }
  updateEqOnBtn(btn: HTMLButtonElement): void {
    this.eqOnBtn = btn;
  }
  updateEqOffBtn(btn: HTMLButtonElement): void {
    this.eqOffBtn = btn;
  }
  updateCancelOptimizationBtn(btn: HTMLButtonElement): void {
    this.cancelOptimizationBtn = btn;
  }
  getAudioStatus(): HTMLElement {
    return this.audioStatus;
  }
  getAudioStatusText(): HTMLElement {
    return this.audioStatusText;
  }
  getAudioDuration(): HTMLElement {
    return this.audioDuration;
  }
  getAudioPosition(): HTMLElement {
    return this.audioPosition;
  }
  getAudioProgressFill(): HTMLElement {
    return this.audioProgressFill;
  }

  // Capture elements
  getCaptureBtn(): HTMLButtonElement | null {
    return this.captureBtn;
  }
  getCaptureStatus(): HTMLElement | null {
    return this.captureStatus;
  }
  getCaptureStatusText(): HTMLElement | null {
    return this.captureStatusText;
  }
  getCaptureProgressFill(): HTMLElement | null {
    return this.captureProgressFill;
  }
  getCaptureWaveform(): HTMLCanvasElement | null {
    return this.captureWaveform;
  }
  getCaptureWaveformCtx(): CanvasRenderingContext2D | null {
    return this.captureWaveformCtx;
  }
  getCaptureResult(): HTMLElement | null {
    return this.captureResult;
  }
  getCaptureClearBtn(): HTMLButtonElement | null {
    return this.captureClearBtn;
  }
  getCapturePlot(): HTMLElement | null {
    return this.capturePlot;
  }

  // State getters
  isEQEnabled(): boolean {
    return this.eqEnabled;
  }

  // Audio control methods
  setAudioStatus(status: string): void {
    console.log("setAudioStatus called with:", status);
    if (this.audioStatusText) {
      this.audioStatusText.textContent = status;
      console.log("Audio status updated to:", status);
    } else {
      console.warn("Audio status text element not found!");
    }
  }

  setListenButtonEnabled(enabled: boolean): void {
    if (this.listenBtn) {
      this.listenBtn.disabled = !enabled;
      if (enabled) {
        this.listenBtn.classList.remove("disabled");
      } else {
        this.listenBtn.classList.add("disabled");
      }
    } else {
      console.warn("Listen button not found in UIManager!");
    }
  }
}
