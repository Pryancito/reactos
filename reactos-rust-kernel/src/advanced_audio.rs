//! Sistema de Audio Avanzado
//!
//! Sistema completo de audio con múltiples formatos, efectos, mezcladores y capacidades avanzadas

use alloc::{vec, vec::Vec, string::{String, ToString}, format, collections::BTreeMap};

/// Formato de audio
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioFormat {
    PCM8,       // 8-bit PCM
    PCM16,      // 16-bit PCM
    PCM24,      // 24-bit PCM
    PCM32,      // 32-bit PCM
    Float32,    // 32-bit float
    Float64,    // 64-bit float
    MP3,        // MP3 compressed
    OGG,        // OGG Vorbis
    WAV,        // WAV container
    FLAC,       // FLAC lossless
}

/// Configuración de canales de audio
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioChannelConfig {
    Mono,       // 1 canal
    Stereo,     // 2 canales
    Quad,       // 4 canales
    Surround5,  // 5.1 surround
    Surround7,  // 7.1 surround
}

/// Estado del reproductor
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaybackState {
    Stopped,    // Detenido
    Playing,    // Reproduciendo
    Paused,     // Pausado
    Buffering,  // Cargando buffer
    Error,      // Error
}

/// Tipo de efecto de audio
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioEffectType {
    Reverb,     // Reverberación
    Echo,       // Eco
    Chorus,     // Coro
    Flanger,    // Flanger
    Distortion, // Distorsión
    Filter,     // Filtro
    Compressor, // Compresor
    Equalizer,  // Ecualizador
    Delay,      // Retraso
    Phaser,     // Phaser
}

/// Configuración de audio
#[derive(Debug, Clone)]
pub struct AudioConfig {
    pub sample_rate: u32,        // Frecuencia de muestreo (Hz)
    pub channels: AudioChannelConfig,  // Número de canales
    pub format: AudioFormat,     // Formato de audio
    pub buffer_size: usize,      // Tamaño del buffer
    pub latency_ms: u32,         // Latencia en milisegundos
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            sample_rate: 44100,
            channels: AudioChannelConfig::Stereo,
            format: AudioFormat::PCM16,
            buffer_size: 4096,
            latency_ms: 100,
        }
    }
}

/// Muestra de audio
#[derive(Debug, Clone)]
pub struct AudioSample {
    pub left: f32,   // Canal izquierdo
    pub right: f32,  // Canal derecho
    pub timestamp: u64, // Timestamp de la muestra
}

impl AudioSample {
    pub fn new(left: f32, right: f32, timestamp: u64) -> Self {
        Self { left, right, timestamp }
    }

    pub fn silence(timestamp: u64) -> Self {
        Self { left: 0.0, right: 0.0, timestamp }
    }

    pub fn mono(value: f32, timestamp: u64) -> Self {
        Self { left: value, right: value, timestamp }
    }
}

/// Buffer de audio circular
#[derive(Debug, Clone)]
pub struct AudioBuffer {
    pub samples: Vec<AudioSample>,
    pub write_pos: usize,
    pub read_pos: usize,
    pub size: usize,
    pub is_full: bool,
}

impl AudioBuffer {
    pub fn new(size: usize) -> Self {
        Self {
            samples: vec![AudioSample::silence(0); size],
            write_pos: 0,
            read_pos: 0,
            size,
            is_full: false,
        }
    }

    pub fn write(&mut self, sample: AudioSample) -> bool {
        if self.is_full && self.write_pos == self.read_pos {
            return false; // Buffer lleno
        }

        self.samples[self.write_pos] = sample;
        self.write_pos = (self.write_pos + 1) % self.size;

        if self.write_pos == self.read_pos {
            self.is_full = true;
        }

        true
    }

    pub fn read(&mut self) -> Option<AudioSample> {
        if !self.is_full && self.read_pos == self.write_pos {
            return None; // Buffer vacío
        }

        let sample = self.samples[self.read_pos].clone();
        self.read_pos = (self.read_pos + 1) % self.size;
        self.is_full = false;

        Some(sample)
    }

    pub fn available_samples(&self) -> usize {
        if self.is_full {
            self.size
        } else if self.write_pos >= self.read_pos {
            self.write_pos - self.read_pos
        } else {
            self.size - self.read_pos + self.write_pos
        }
    }

    pub fn clear(&mut self) {
        self.write_pos = 0;
        self.read_pos = 0;
        self.is_full = false;
    }
}

/// Efecto de audio
#[derive(Debug, Clone)]
pub struct AudioEffect {
    pub id: usize,
    pub effect_type: AudioEffectType,
    pub enabled: bool,
    pub parameters: BTreeMap<String, f32>,
    pub buffer: Vec<f32>,
    pub buffer_size: usize,
}

impl AudioEffect {
    pub fn new(id: usize, effect_type: AudioEffectType) -> Self {
        let mut effect = Self {
            id,
            effect_type,
            enabled: true,
            parameters: BTreeMap::new(),
            buffer: Vec::new(),
            buffer_size: 1024,
        };

        // Inicializar parámetros por defecto según el tipo de efecto
        match effect_type {
            AudioEffectType::Reverb => {
                effect.parameters.insert("room_size".to_string(), 0.5);
                effect.parameters.insert("damping".to_string(), 0.5);
                effect.parameters.insert("wet_level".to_string(), 0.3);
                effect.parameters.insert("dry_level".to_string(), 0.7);
            },
            AudioEffectType::Echo => {
                effect.parameters.insert("delay".to_string(), 0.3);
                effect.parameters.insert("feedback".to_string(), 0.4);
                effect.parameters.insert("wet_level".to_string(), 0.3);
            },
            AudioEffectType::Chorus => {
                effect.parameters.insert("rate".to_string(), 0.5);
                effect.parameters.insert("depth".to_string(), 0.5);
                effect.parameters.insert("mix".to_string(), 0.5);
            },
            AudioEffectType::Flanger => {
                effect.parameters.insert("rate".to_string(), 0.2);
                effect.parameters.insert("depth".to_string(), 0.7);
                effect.parameters.insert("feedback".to_string(), 0.3);
            },
            AudioEffectType::Distortion => {
                effect.parameters.insert("gain".to_string(), 2.0);
                effect.parameters.insert("tone".to_string(), 0.5);
                effect.parameters.insert("level".to_string(), 0.7);
            },
            AudioEffectType::Filter => {
                effect.parameters.insert("cutoff".to_string(), 0.5);
                effect.parameters.insert("resonance".to_string(), 0.1);
                effect.parameters.insert("type".to_string(), 0.0); // 0=lowpass, 1=highpass, 2=bandpass
            },
            AudioEffectType::Compressor => {
                effect.parameters.insert("threshold".to_string(), 0.7);
                effect.parameters.insert("ratio".to_string(), 4.0);
                effect.parameters.insert("attack".to_string(), 0.01);
                effect.parameters.insert("release".to_string(), 0.1);
            },
            AudioEffectType::Equalizer => {
                effect.parameters.insert("low_gain".to_string(), 0.0);
                effect.parameters.insert("mid_gain".to_string(), 0.0);
                effect.parameters.insert("high_gain".to_string(), 0.0);
            },
            AudioEffectType::Delay => {
                effect.parameters.insert("delay_time".to_string(), 0.25);
                effect.parameters.insert("feedback".to_string(), 0.3);
                effect.parameters.insert("mix".to_string(), 0.3);
            },
            AudioEffectType::Phaser => {
                effect.parameters.insert("rate".to_string(), 0.5);
                effect.parameters.insert("depth".to_string(), 0.8);
                effect.parameters.insert("feedback".to_string(), 0.2);
            },
        }

        effect.buffer = vec![0.0; effect.buffer_size];
        effect
    }

    pub fn process_sample(&mut self, input: f32) -> f32 {
        if !self.enabled {
            return input;
        }

        match self.effect_type {
            AudioEffectType::Reverb => self.process_reverb(input),
            AudioEffectType::Echo => self.process_echo(input),
            AudioEffectType::Chorus => self.process_chorus(input),
            AudioEffectType::Flanger => self.process_flanger(input),
            AudioEffectType::Distortion => self.process_distortion(input),
            AudioEffectType::Filter => self.process_filter(input),
            AudioEffectType::Compressor => self.process_compressor(input),
            AudioEffectType::Equalizer => self.process_equalizer(input),
            AudioEffectType::Delay => self.process_delay(input),
            AudioEffectType::Phaser => self.process_phaser(input),
        }
    }

    fn process_reverb(&mut self, input: f32) -> f32 {
        let room_size = self.parameters.get("room_size").unwrap_or(&0.5);
        let damping = self.parameters.get("damping").unwrap_or(&0.5);
        let wet_level = self.parameters.get("wet_level").unwrap_or(&0.3);
        let dry_level = self.parameters.get("dry_level").unwrap_or(&0.7);

        // Simulación simple de reverb
        let reverb = input * *room_size * (1.0 - *damping);
        input * *dry_level + reverb * *wet_level
    }

    fn process_echo(&mut self, input: f32) -> f32 {
        let delay = self.parameters.get("delay").unwrap_or(&0.3);
        let feedback = self.parameters.get("feedback").unwrap_or(&0.4);
        let wet_level = self.parameters.get("wet_level").unwrap_or(&0.3);

        // Simulación simple de echo
        let echo_delay = (*delay * 44100.0) as usize;
        if echo_delay < self.buffer_size {
            let delayed = self.buffer[echo_delay];
            self.buffer[echo_delay] = input + delayed * *feedback;
            input + delayed * *wet_level
        } else {
            input
        }
    }

    fn process_chorus(&mut self, input: f32) -> f32 {
        let rate = self.parameters.get("rate").unwrap_or(&0.5);
        let depth = self.parameters.get("depth").unwrap_or(&0.5);
        let mix = self.parameters.get("mix").unwrap_or(&0.5);

        // Simulación simple de chorus
        let modulation = simple_sin(self.buffer[0] * *rate) * *depth;
        let chorus = input + modulation;
        input * (1.0 - *mix) + chorus * *mix
    }

    fn process_flanger(&mut self, input: f32) -> f32 {
        let rate = self.parameters.get("rate").unwrap_or(&0.2);
        let depth = self.parameters.get("depth").unwrap_or(&0.7);
        let feedback = self.parameters.get("feedback").unwrap_or(&0.3);

        // Simulación simple de flanger
        let delay = (*depth * 0.01) as usize;
        if delay < self.buffer_size {
            let delayed = self.buffer[delay];
            self.buffer[delay] = input + delayed * *feedback;
            input + delayed
        } else {
            input
        }
    }

    fn process_distortion(&mut self, input: f32) -> f32 {
        let gain = self.parameters.get("gain").unwrap_or(&2.0);
        let tone = self.parameters.get("tone").unwrap_or(&0.5);
        let level = self.parameters.get("level").unwrap_or(&0.7);

        // Simulación simple de distorsión
        let distorted = simple_tanh(input * *gain) * *tone;
        distorted * *level
    }

    fn process_filter(&mut self, input: f32) -> f32 {
        let cutoff = self.parameters.get("cutoff").unwrap_or(&0.5);
        let resonance = self.parameters.get("resonance").unwrap_or(&0.1);
        let filter_type = self.parameters.get("type").unwrap_or(&0.0);

        // Simulación simple de filtro
        match *filter_type as u32 {
            0 => input * *cutoff, // Low pass
            1 => input * (1.0 - *cutoff), // High pass
            2 => input * *cutoff * (1.0 - *cutoff), // Band pass
            _ => input,
        }
    }

    fn process_compressor(&mut self, input: f32) -> f32 {
        let threshold = self.parameters.get("threshold").unwrap_or(&0.7);
        let ratio = self.parameters.get("ratio").unwrap_or(&4.0);
        let attack = self.parameters.get("attack").unwrap_or(&0.01);
        let release = self.parameters.get("release").unwrap_or(&0.1);

        // Simulación simple de compresor
        let abs_input = input.abs();
        if abs_input > *threshold {
            let compressed = *threshold + (abs_input - *threshold) / *ratio;
            compressed * input.signum()
        } else {
            input
        }
    }

    fn process_equalizer(&mut self, input: f32) -> f32 {
        let low_gain = self.parameters.get("low_gain").unwrap_or(&0.0);
        let mid_gain = self.parameters.get("mid_gain").unwrap_or(&0.0);
        let high_gain = self.parameters.get("high_gain").unwrap_or(&0.0);

        // Simulación simple de ecualizador
        input * (1.0 + *low_gain + *mid_gain + *high_gain)
    }

    fn process_delay(&mut self, input: f32) -> f32 {
        let delay_time = self.parameters.get("delay_time").unwrap_or(&0.25);
        let feedback = self.parameters.get("feedback").unwrap_or(&0.3);
        let mix = self.parameters.get("mix").unwrap_or(&0.3);

        // Simulación simple de delay
        let delay_samples = (*delay_time * 44100.0) as usize;
        if delay_samples < self.buffer_size {
            let delayed = self.buffer[delay_samples];
            self.buffer[delay_samples] = input + delayed * *feedback;
            input * (1.0 - *mix) + delayed * *mix
        } else {
            input
        }
    }

    fn process_phaser(&mut self, input: f32) -> f32 {
        let rate = self.parameters.get("rate").unwrap_or(&0.5);
        let depth = self.parameters.get("depth").unwrap_or(&0.8);
        let feedback = self.parameters.get("feedback").unwrap_or(&0.2);

        // Simulación simple de phaser
        let modulation = simple_sin(self.buffer[0] * *rate) * *depth;
        let phased = input + modulation;
        self.buffer[0] = phased * *feedback;
        phased
    }
}

/// Canal de audio
#[derive(Debug, Clone)]
pub struct AudioChannel {
    pub id: usize,
    pub name: String,
    pub volume: f32,         // 0.0 - 1.0
    pub pan: f32,           // -1.0 (izquierda) a 1.0 (derecha)
    pub mute: bool,
    pub solo: bool,
    pub effects: Vec<AudioEffect>,
    pub buffer: AudioBuffer,
}

impl AudioChannel {
    pub fn new(id: usize, name: String) -> Self {
        Self {
            id,
            name,
            volume: 1.0,
            pan: 0.0,
            mute: false,
            solo: false,
            effects: Vec::new(),
            buffer: AudioBuffer::new(4096),
        }
    }

    pub fn add_effect(&mut self, effect_type: AudioEffectType) -> usize {
        let effect_id = self.effects.len();
        let effect = AudioEffect::new(effect_id, effect_type);
        self.effects.push(effect);
        effect_id
    }

    pub fn remove_effect(&mut self, effect_id: usize) -> bool {
        if effect_id < self.effects.len() {
            self.effects.remove(effect_id);
            true
        } else {
            false
        }
    }

    pub fn process_sample(&mut self, input: AudioSample) -> AudioSample {
        if self.mute {
            return AudioSample::silence(input.timestamp);
        }

        let mut left = input.left * self.volume;
        let mut right = input.right * self.volume;

        // Aplicar efectos
        for effect in &mut self.effects {
            left = effect.process_sample(left);
            right = effect.process_sample(right);
        }

        // Aplicar pan
        let pan_left = if self.pan <= 0.0 { 1.0 } else { 1.0 - self.pan };
        let pan_right = if self.pan >= 0.0 { 1.0 } else { 1.0 + self.pan };

        left *= pan_left;
        right *= pan_right;

        AudioSample::new(left, right, input.timestamp)
    }
}

/// Mezclador de audio
#[derive(Debug, Clone)]
pub struct AudioMixer {
    pub channels: Vec<AudioChannel>,
    pub master_volume: f32,
    pub master_mute: bool,
    pub output_buffer: AudioBuffer,
    pub sample_rate: u32,
}

impl AudioMixer {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            channels: Vec::new(),
            master_volume: 1.0,
            master_mute: false,
            output_buffer: AudioBuffer::new(8192),
            sample_rate,
        }
    }

    pub fn add_channel(&mut self, name: String) -> usize {
        let channel_id = self.channels.len();
        let channel = AudioChannel::new(channel_id, name);
        self.channels.push(channel);
        channel_id
    }

    pub fn remove_channel(&mut self, channel_id: usize) -> bool {
        if channel_id < self.channels.len() {
            self.channels.remove(channel_id);
            // Reasignar IDs
            for (i, channel) in self.channels.iter_mut().enumerate() {
                channel.id = i;
            }
            true
        } else {
            false
        }
    }

    pub fn get_channel(&mut self, channel_id: usize) -> Option<&mut AudioChannel> {
        self.channels.get_mut(channel_id)
    }

    pub fn mix_samples(&mut self, timestamp: u64) -> AudioSample {
        if self.master_mute {
            return AudioSample::silence(timestamp);
        }

        let mut mixed_left = 0.0;
        let mut mixed_right = 0.0;
        let mut active_channels = 0;

        // Mezclar todos los canales
        for channel in &mut self.channels {
            if let Some(sample) = channel.buffer.read() {
                let processed = channel.process_sample(sample);
                mixed_left += processed.left;
                mixed_right += processed.right;
                active_channels += 1;
            }
        }

        // Aplicar volumen maestro
        mixed_left *= self.master_volume;
        mixed_right *= self.master_volume;

        // Normalizar si hay muchos canales activos
        if active_channels > 1 {
            let normalization = 1.0 / simple_sqrt(active_channels as f32);
            mixed_left *= normalization;
            mixed_right *= normalization;
        }

        AudioSample::new(mixed_left, mixed_right, timestamp)
    }
}

/// Reproductor de audio
#[derive(Debug, Clone)]
pub struct AudioPlayer {
    pub id: usize,
    pub name: String,
    pub state: PlaybackState,
    pub config: AudioConfig,
    pub position: u64,      // Posición actual en muestras
    pub duration: u64,      // Duración total en muestras
    pub loop_enabled: bool,
    pub fade_in_ms: u32,
    pub fade_out_ms: u32,
    pub buffer: AudioBuffer,
}

impl AudioPlayer {
    pub fn new(id: usize, name: String, config: AudioConfig) -> Self {
        let buffer_size = config.buffer_size;
        Self {
            id,
            name,
            state: PlaybackState::Stopped,
            config,
            position: 0,
            duration: 0,
            loop_enabled: false,
            fade_in_ms: 0,
            fade_out_ms: 0,
            buffer: AudioBuffer::new(buffer_size),
        }
    }

    pub fn play(&mut self) {
        self.state = PlaybackState::Playing;
    }

    pub fn pause(&mut self) {
        if self.state == PlaybackState::Playing {
            self.state = PlaybackState::Paused;
        }
    }

    pub fn stop(&mut self) {
        self.state = PlaybackState::Stopped;
        self.position = 0;
        self.buffer.clear();
    }

    pub fn seek(&mut self, position: u64) -> bool {
        if position <= self.duration {
            self.position = position;
            true
        } else {
            false
        }
    }

    pub fn get_position_ms(&self) -> u32 {
        ((self.position * 1000) / self.config.sample_rate as u64) as u32
    }

    pub fn get_duration_ms(&self) -> u32 {
        ((self.duration * 1000) / self.config.sample_rate as u64) as u32
    }

    pub fn get_progress(&self) -> f32 {
        if self.duration > 0 {
            self.position as f32 / self.duration as f32
        } else {
            0.0
        }
    }
}

/// Gestor de audio avanzado
#[derive(Debug, Clone)]
pub struct AdvancedAudioManager {
    pub mixer: AudioMixer,
    pub players: Vec<AudioPlayer>,
    pub next_player_id: usize,
    pub next_effect_id: usize,
    pub statistics: AudioStatistics,
    pub is_initialized: bool,
}

#[derive(Debug, Clone)]
pub struct AudioStatistics {
    pub total_samples_processed: u64,
    pub total_playback_time_ms: u64,
    pub active_players: usize,
    pub active_effects: usize,
    pub buffer_underruns: u32,
    pub buffer_overruns: u32,
    pub average_latency_ms: f32,
    pub peak_volume: f32,
}

impl Default for AudioStatistics {
    fn default() -> Self {
        Self {
            total_samples_processed: 0,
            total_playback_time_ms: 0,
            active_players: 0,
            active_effects: 0,
            buffer_underruns: 0,
            buffer_overruns: 0,
            average_latency_ms: 0.0,
            peak_volume: 0.0,
        }
    }
}

impl AdvancedAudioManager {
    pub fn new() -> Self {
        Self {
            mixer: AudioMixer::new(44100),
            players: Vec::new(),
            next_player_id: 1,
            next_effect_id: 1,
            statistics: AudioStatistics::default(),
            is_initialized: false,
        }
    }

    pub fn initialize(&mut self) -> bool {
        // Inicializar canales por defecto
        self.mixer.add_channel("Master".to_string());
        self.mixer.add_channel("Music".to_string());
        self.mixer.add_channel("SFX".to_string());
        self.mixer.add_channel("Voice".to_string());

        self.is_initialized = true;
        true
    }

    pub fn create_player(&mut self, name: String, config: AudioConfig) -> usize {
        let player_id = self.next_player_id;
        let player = AudioPlayer::new(player_id, name, config.clone());
        self.players.push(player);
        self.next_player_id += 1;
        player_id
    }

    pub fn remove_player(&mut self, player_id: usize) -> bool {
        if let Some(pos) = self.players.iter().position(|p| p.id == player_id) {
            self.players.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn get_player(&mut self, player_id: usize) -> Option<&mut AudioPlayer> {
        self.players.iter_mut().find(|p| p.id == player_id)
    }

    pub fn play_tone(&mut self, frequency: f32, duration_ms: u32, volume: f32) -> usize {
        let config = AudioConfig::default();
        let player_id = self.create_player(
            format!("Tone_{}Hz", frequency),
            config.clone()
        );

        if let Some(player) = self.get_player(player_id) {
            player.duration = (duration_ms as u64 * config.sample_rate as u64) / 1000;
            player.play();
        }

        player_id
    }

    pub fn play_melody(&mut self, notes: Vec<(f32, u32)>, volume: f32) -> usize {
        let config = AudioConfig::default();
        let player_id = self.create_player("Melody".to_string(), config.clone());

        if let Some(player) = self.get_player(player_id) {
            let total_duration: u64 = notes.iter().map(|(_, duration)| *duration as u64).sum();
            player.duration = (total_duration * config.sample_rate as u64) / 1000;
            player.play();
        }

        player_id
    }

    pub fn process_audio(&mut self) {
        if !self.is_initialized {
            return;
        }

        let timestamp = get_audio_timestamp();
        let mixed_sample = self.mixer.mix_samples(timestamp);

        // Actualizar estadísticas
        self.statistics.total_samples_processed += 1;
        self.statistics.peak_volume = self.statistics.peak_volume.max(
            mixed_sample.left.abs().max(mixed_sample.right.abs())
        );

        // Procesar reproductores activos
        self.statistics.active_players = 0;
        for player in &mut self.players {
            if player.state == PlaybackState::Playing {
                self.statistics.active_players += 1;
                // Simular generación de audio
                player.position += 1;
                if player.position >= player.duration {
                    if player.loop_enabled {
                        player.position = 0;
                    } else {
                        player.state = PlaybackState::Stopped;
                    }
                }
            }
        }

        // Contar efectos activos
        self.statistics.active_effects = 0;
        for channel in &self.mixer.channels {
            for effect in &channel.effects {
                if effect.enabled {
                    self.statistics.active_effects += 1;
                }
            }
        }
    }

    pub fn get_info(&self) -> String {
        format!(
            "Audio Avanzado - Reproductores: {} | Canales: {} | Efectos: {} | Estado: {}",
            self.players.len(),
            self.mixer.channels.len(),
            self.statistics.active_effects,
            if self.is_initialized { "Inicializado" } else { "No inicializado" }
        )
    }

    pub fn get_statistics(&self) -> String {
        format!(
            "Muestras: {} | Tiempo: {}ms | Activos: {} | Efectos: {} | Pico: {:.2} | Latencia: {:.1}ms",
            self.statistics.total_samples_processed,
            self.statistics.total_playback_time_ms,
            self.statistics.active_players,
            self.statistics.active_effects,
            self.statistics.peak_volume,
            self.statistics.average_latency_ms
        )
    }
}

// Funciones auxiliares
fn get_audio_timestamp() -> u64 {
    // Simulación simple de timestamp de audio
    1234567890
}

// Funciones matemáticas simples para no_std
fn simple_sin(x: f32) -> f32 {
    // Aproximación simple de seno usando serie de Taylor
    let x = x % (2.0 * 3.14159);
    let x2 = x * x;
    let x3 = x2 * x;
    let x5 = x3 * x2;
    let x7 = x5 * x2;
    x - x3/6.0 + x5/120.0 - x7/5040.0
}

fn simple_tanh(x: f32) -> f32 {
    // Aproximación simple de tangente hiperbólica
    if x > 3.0 { 1.0 }
    else if x < -3.0 { -1.0 }
    else { x / (1.0 + x.abs()) }
}

fn simple_sqrt(x: f32) -> f32 {
    // Aproximación simple de raíz cuadrada usando método de Newton
    if x <= 0.0 { return 0.0; }
    let mut guess = x / 2.0;
    for _ in 0..10 {
        guess = (guess + x / guess) / 2.0;
    }
    guess
}

// Gestor global de audio avanzado
use spin::Mutex;

pub static ADVANCED_AUDIO_MANAGER: Mutex<Option<AdvancedAudioManager>> = Mutex::new(None);

/// Inicializar el gestor de audio avanzado
pub fn init_advanced_audio() {
    let mut manager = ADVANCED_AUDIO_MANAGER.lock();
    *manager = Some(AdvancedAudioManager::new());
    if let Some(ref mut am) = *manager {
        am.initialize();
    }
    crate::logging::info("advanced_audio", "Sistema de audio avanzado inicializado");
}

/// Obtener información del gestor de audio
pub fn get_advanced_audio_info() -> String {
    if let Some(ref manager) = *ADVANCED_AUDIO_MANAGER.lock() {
        manager.get_info()
    } else {
        String::from("Sistema de audio avanzado no inicializado")
    }
}

/// Obtener estadísticas del gestor de audio
pub fn get_advanced_audio_stats() -> String {
    if let Some(ref manager) = *ADVANCED_AUDIO_MANAGER.lock() {
        manager.get_statistics()
    } else {
        String::from("Sistema de audio avanzado no inicializado")
    }
}

/// Reproducir tono
pub fn play_tone(frequency: f32, duration_ms: u32, volume: f32) -> usize {
    let mut manager = ADVANCED_AUDIO_MANAGER.lock();
    if let Some(ref mut am) = *manager {
        am.play_tone(frequency, duration_ms, volume)
    } else {
        0
    }
}

/// Reproducir melodía
pub fn play_melody(notes: Vec<(f32, u32)>, volume: f32) -> usize {
    let mut manager = ADVANCED_AUDIO_MANAGER.lock();
    if let Some(ref mut am) = *manager {
        am.play_melody(notes, volume)
    } else {
        0
    }
}

/// Procesar audio
pub fn process_audio() {
    let mut manager = ADVANCED_AUDIO_MANAGER.lock();
    if let Some(ref mut am) = *manager {
        am.process_audio();
    }
}

/// Verificar si el sistema de audio avanzado está disponible
pub fn is_advanced_audio_available() -> bool {
    let manager = ADVANCED_AUDIO_MANAGER.lock();
    manager.is_some()
}
