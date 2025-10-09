/**
 * Audio Device Manager - Integrates cpal-based Tauri API with WebAudio
 * Provides unified interface for device enumeration and configuration
 */

import {
  getAudioDevices,
  setAudioDevice,
  getAudioConfig,
  getDeviceProperties,
  type AudioDevice,
  type AudioConfig,
  type AudioState,
  type AudioDevicesMap
} from './audio-interface';

export interface UnifiedAudioDevice {
  deviceId: string;
  name: string;
  type: 'input' | 'output';
  isDefault: boolean;
  isWebAudio: boolean; // true if from WebAudio API, false if from cpal
  channels: number;
  sampleRates: number[];
  defaultSampleRate?: number;
  formats: string[];
  webAudioDevice?: MediaDeviceInfo; // Original WebAudio device if applicable
  cpalDevice?: AudioDevice; // Original cpal device if applicable
}

export interface DeviceSelectionResult {
  success: boolean;
  device?: UnifiedAudioDevice;
  config?: AudioConfig;
  error?: string;
}

/**
 * Enhanced Audio Device Manager that combines cpal and WebAudio capabilities
 */
export class AudioDeviceManager {
  private webAudioDevices: Map<string, MediaDeviceInfo> = new Map();
  private cpalDevices: Map<string, AudioDevice> = new Map();
  private unifiedDevices: Map<string, UnifiedAudioDevice> = new Map();
  private currentState: AudioState | null = null;
  private preferCpal: boolean = true; // Prefer cpal devices when available

  constructor(preferCpal: boolean = true) {
    this.preferCpal = preferCpal;
  }

  /**
   * Enumerate all available audio devices from both WebAudio and cpal
   */
  async enumerateDevices(): Promise<{
    input: UnifiedAudioDevice[];
    output: UnifiedAudioDevice[];
  }> {
    console.log('[DeviceManager] Enumerating devices from all sources...');
    
    // Clear previous device maps
    this.webAudioDevices.clear();
    this.cpalDevices.clear();
    this.unifiedDevices.clear();

    const inputDevices: UnifiedAudioDevice[] = [];
    const outputDevices: UnifiedAudioDevice[] = [];

    // Get cpal devices first (if available)
    try {
      const cpalDeviceMap = await getAudioDevices();
      console.log('[DeviceManager] Got cpal devices:', {
        input: cpalDeviceMap.input.length,
        output: cpalDeviceMap.output.length
      });

      // Process cpal input devices
      for (const device of cpalDeviceMap.input) {
        const unifiedDevice = this.cpalToUnified(device, 'input');
        this.cpalDevices.set(unifiedDevice.deviceId, device);
        this.unifiedDevices.set(unifiedDevice.deviceId, unifiedDevice);
        inputDevices.push(unifiedDevice);
      }

      // Process cpal output devices
      for (const device of cpalDeviceMap.output) {
        const unifiedDevice = this.cpalToUnified(device, 'output');
        this.cpalDevices.set(unifiedDevice.deviceId, device);
        this.unifiedDevices.set(unifiedDevice.deviceId, unifiedDevice);
        outputDevices.push(unifiedDevice);
      }
    } catch (error) {
      console.warn('[DeviceManager] Could not get cpal devices:', error);
      console.log('[DeviceManager] Falling back to WebAudio only');
    }

    // Get WebAudio devices as fallback or supplement
    try {
      const webDevices = await navigator.mediaDevices.enumerateDevices();
      console.log('[DeviceManager] Got WebAudio devices:', webDevices.length);

      for (const device of webDevices) {
        // Skip non-audio devices
        if (device.kind !== 'audioinput' && device.kind !== 'audiooutput') {
          continue;
        }

        const type = device.kind === 'audioinput' ? 'input' : 'output';
        
        // Check if we already have this device from cpal (match by name)
        const existingDevice = Array.from(this.unifiedDevices.values()).find(
          d => d.name === device.label && d.type === type
        );

        if (existingDevice && this.preferCpal) {
          // Attach WebAudio info to existing cpal device
          existingDevice.webAudioDevice = device;
          continue;
        }

        // Create unified device from WebAudio
        const unifiedDevice = await this.webAudioToUnified(device);
        this.webAudioDevices.set(device.deviceId, device);
        this.unifiedDevices.set(device.deviceId, unifiedDevice);

        if (type === 'input') {
          inputDevices.push(unifiedDevice);
        } else {
          outputDevices.push(unifiedDevice);
        }
      }
    } catch (error) {
      console.error('[DeviceManager] Error enumerating WebAudio devices:', error);
    }

    console.log('[DeviceManager] Total unified devices:', {
      input: inputDevices.length,
      output: outputDevices.length
    });

    return { input: inputDevices, output: outputDevices };
  }

  /**
   * Convert cpal device to unified format
   */
  private cpalToUnified(device: AudioDevice, type: 'input' | 'output'): UnifiedAudioDevice {
    // Extract unique sample rates from supported configs
    const sampleRates = Array.from(
      new Set(device.supported_configs.map(c => c.sample_rate))
    ).sort((a, b) => a - b);

    // Get max channel count
    const maxChannels = Math.max(
      ...device.supported_configs.map(c => c.channels),
      device.default_config?.channels || 2
    );

    // Extract unique formats
    const formats = Array.from(
      new Set(device.supported_configs.map(c => c.sample_format))
    );

    return {
      deviceId: `cpal_${type}_${device.name.replace(/\s+/g, '_')}`,
      name: device.name,
      type,
      isDefault: device.is_default,
      isWebAudio: false,
      channels: maxChannels,
      sampleRates,
      defaultSampleRate: device.default_config?.sample_rate,
      formats,
      cpalDevice: device
    };
  }

  /**
   * Convert WebAudio device to unified format
   */
  private async webAudioToUnified(device: MediaDeviceInfo): Promise<UnifiedAudioDevice> {
    const type = device.kind === 'audioinput' ? 'input' : 'output';
    
    // Try to get device capabilities
    let channels = 2; // Default assumption
    let sampleRate = 48000; // Default assumption

    if (type === 'input') {
      try {
        // Try to get actual capabilities
        const stream = await navigator.mediaDevices.getUserMedia({
          audio: {
            deviceId: { exact: device.deviceId },
            channelCount: { ideal: 32 }
          }
        });

        const track = stream.getAudioTracks()[0];
        const settings = track.getSettings();
        
        if (settings.channelCount) {
          channels = settings.channelCount;
        }

        // Get sample rate from audio context
        const audioContext = new AudioContext();
        sampleRate = audioContext.sampleRate;
        audioContext.close();

        // Clean up stream
        stream.getTracks().forEach(t => t.stop());
      } catch (error) {
        console.warn('[DeviceManager] Could not probe WebAudio device:', device.label, error);
      }
    }

    return {
      deviceId: device.deviceId,
      name: device.label || `${type === 'input' ? 'Microphone' : 'Speaker'} ${device.deviceId.substr(0, 8)}`,
      type,
      isDefault: device.deviceId === 'default',
      isWebAudio: true,
      channels,
      sampleRates: [sampleRate],
      defaultSampleRate: sampleRate,
      formats: ['f32'], // WebAudio typically uses float32
      webAudioDevice: device
    };
  }

  /**
   * Select and configure an audio device
   */
  async selectDevice(
    deviceId: string,
    config?: Partial<AudioConfig>
  ): Promise<DeviceSelectionResult> {
    const device = this.unifiedDevices.get(deviceId);
    
    if (!device) {
      return {
        success: false,
        error: `Device ${deviceId} not found`
      };
    }

    console.log('[DeviceManager] Selecting device:', device.name, config);

    // If it's a cpal device, configure it through Tauri
    if (device.cpalDevice) {
      try {
        // Build config with defaults
        const fullConfig: AudioConfig = {
          sample_rate: config?.sample_rate || device.defaultSampleRate || 48000,
          channels: config?.channels || Math.min(2, device.channels),
          buffer_size: config?.buffer_size,
          sample_format: (config?.sample_format || device.formats[0] || 'f32') as any
        };

        // Set the device configuration via Tauri
        const result = await setAudioDevice(
          device.cpalDevice.name,
          device.type === 'input',
          fullConfig
        );

        console.log('[DeviceManager] Device configured:', result);

        return {
          success: true,
          device,
          config: fullConfig
        };
      } catch (error) {
        console.error('[DeviceManager] Error configuring cpal device:', error);
        return {
          success: false,
          error: String(error)
        };
      }
    }

    // For WebAudio devices, just return success as they're configured on use
    return {
      success: true,
      device,
      config: {
        sample_rate: device.defaultSampleRate || 48000,
        channels: Math.min(config?.channels || 2, device.channels),
        buffer_size: config?.buffer_size,
        sample_format: 'f32'
      }
    };
  }

  /**
   * Get current audio configuration state
   */
  async getCurrentState(): Promise<AudioState | null> {
    try {
      this.currentState = await getAudioConfig();
      return this.currentState;
    } catch (error) {
      console.error('[DeviceManager] Error getting audio state:', error);
      return null;
    }
  }

  /**
   * Get detailed properties for a specific device
   */
  async getDeviceDetails(deviceId: string): Promise<any> {
    const device = this.unifiedDevices.get(deviceId);
    
    if (!device) {
      throw new Error(`Device ${deviceId} not found`);
    }

    // If it's a cpal device, get detailed properties from Tauri
    if (device.cpalDevice) {
      try {
        return await getDeviceProperties(
          device.cpalDevice.name,
          device.type === 'input'
        );
      } catch (error) {
        console.error('[DeviceManager] Error getting device properties:', error);
      }
    }

    // Return basic info for WebAudio devices
    return {
      name: device.name,
      type: device.type,
      channels: device.channels,
      sampleRates: device.sampleRates,
      formats: device.formats,
      isWebAudio: true
    };
  }

  /**
   * Find best matching device by criteria
   */
  findBestDevice(
    type: 'input' | 'output',
    criteria?: {
      preferredChannels?: number;
      preferredSampleRate?: number;
      preferDefault?: boolean;
    }
  ): UnifiedAudioDevice | null {
    const devices = Array.from(this.unifiedDevices.values()).filter(
      d => d.type === type
    );

    if (devices.length === 0) {
      return null;
    }

    // If preferring default, return it
    if (criteria?.preferDefault) {
      const defaultDevice = devices.find(d => d.isDefault);
      if (defaultDevice) return defaultDevice;
    }

    // Score devices based on criteria
    let bestDevice = devices[0];
    let bestScore = 0;

    for (const device of devices) {
      let score = 0;

      // Prefer cpal devices
      if (!device.isWebAudio) score += 10;

      // Match channel count
      if (criteria?.preferredChannels) {
        if (device.channels >= criteria.preferredChannels) {
          score += 5;
        }
      }

      // Match sample rate
      if (criteria?.preferredSampleRate) {
        if (device.sampleRates.includes(criteria.preferredSampleRate)) {
          score += 5;
        }
      }

      // Prefer devices with more capabilities
      score += device.sampleRates.length;
      score += device.channels;

      if (score > bestScore) {
        bestScore = score;
        bestDevice = device;
      }
    }

    return bestDevice;
  }

  /**
   * Create UI-friendly device list for dropdowns
   */
  getDeviceList(type: 'input' | 'output'): Array<{ value: string; label: string; info?: string }> {
    const devices = Array.from(this.unifiedDevices.values()).filter(
      d => d.type === type
    );

    return devices.map(device => ({
      value: device.deviceId,
      label: device.name,
      info: `${device.channels}ch ${device.defaultSampleRate ? Math.round(device.defaultSampleRate / 1000) + 'kHz' : ''} ${device.isDefault ? '(Default)' : ''}`
    }));
  }
}