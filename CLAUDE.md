# CHIP-8 Emulator Project Knowledge

This document contains research and specifications for building a CHIP-8 interpreter in Rust.

## CHIP-8 Specification

### Memory Layout
- **Total RAM**: 4 KB (4096 bytes), addresses 0x000 to 0xFFF
- **Reserved**: 0x000–0x1FF (512 bytes) - Originally for interpreter, now used for font data
- **Font Data**: Typically stored at 0x050–0x09F (80 bytes for 16 hex digit sprites)
- **Program Start**: 0x200 (512) - All programs should be loaded starting here
- **Upper Memory**: 0x200–0xFFF available for program code and data

### Registers
| Register | Size | Purpose |
|----------|------|---------|
| V0–VE | 8-bit each | General purpose registers |
| VF | 8-bit | Flag register (carry, borrow, collision) |
| I | 16-bit | Index/address register for memory operations |
| PC | 16-bit | Program counter |
| SP | 8-bit | Stack pointer |
| DT | 8-bit | Delay timer (decrements at 60 Hz) |
| ST | 8-bit | Sound timer (decrements at 60 Hz, beeps when > 0) |

### Stack
- 16 levels deep
- Stores 16-bit return addresses for subroutine calls
- Push on CALL (2NNN), pop on RET (00EE)

### Display
- **Resolution**: 64 × 32 pixels (monochrome)
- **Drawing**: Sprites are XOR'd onto screen
- **Collision**: VF set to 1 if any pixel is flipped from on to off
- **Sprites**: 8 pixels wide, 1–15 pixels tall
- **Wrapping**: Sprites wrap at their starting position coordinates
- **Clipping**: Sprites clip at screen edges

### Timers
Both timers decrement at 60 Hz when non-zero:
- **Delay Timer (DT)**: General purpose timing
- **Sound Timer (ST)**: Produces beep while > 0

### Keypad
Original COSMAC VIP hex keypad layout:
```
1 2 3 C        Modern keyboard mapping:
4 5 6 D   →    1 2 3 4
7 8 9 E        Q W E R
A 0 B F        A S D F
               Z X C V
```

### Font Sprites
Built-in 4×5 pixel sprites for hex digits 0–F. Each character is 5 bytes.
Standard font data (80 bytes total):
```rust
const FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];
```

---

## Complete Opcode Reference

All instructions are 2 bytes, stored big-endian. Fetch at PC, increment PC by 2.

### Notation
- `NNN` - 12-bit address (lower 12 bits)
- `NN` / `KK` - 8-bit constant (lower 8 bits)
- `N` - 4-bit constant (lowest nibble)
- `X` - 4-bit register identifier (second nibble)
- `Y` - 4-bit register identifier (third nibble)

### System
| Opcode | Mnemonic | Description |
|--------|----------|-------------|
| `0NNN` | SYS addr | Machine code routine (ignored on modern interpreters) |
| `00E0` | CLS | Clear the display |
| `00EE` | RET | Return from subroutine (PC = stack[--SP]) |

### Flow Control
| Opcode | Mnemonic | Description |
|--------|----------|-------------|
| `1NNN` | JP addr | Jump to address NNN (PC = NNN) |
| `2NNN` | CALL addr | Call subroutine (stack[SP++] = PC, PC = NNN) |
| `BNNN` | JP V0, addr | Jump to NNN + V0 |

### Conditionals (Skip Instructions)
| Opcode | Mnemonic | Description |
|--------|----------|-------------|
| `3XNN` | SE Vx, byte | Skip if VX == NN |
| `4XNN` | SNE Vx, byte | Skip if VX != NN |
| `5XY0` | SE Vx, Vy | Skip if VX == VY |
| `9XY0` | SNE Vx, Vy | Skip if VX != VY |

### Constants
| Opcode | Mnemonic | Description |
|--------|----------|-------------|
| `6XNN` | LD Vx, byte | VX = NN |
| `7XNN` | ADD Vx, byte | VX += NN (no carry flag) |
| `CXNN` | RND Vx, byte | VX = random() & NN |

### Arithmetic & Logic (8XY_)
| Opcode | Mnemonic | Description |
|--------|----------|-------------|
| `8XY0` | LD Vx, Vy | VX = VY |
| `8XY1` | OR Vx, Vy | VX \|= VY |
| `8XY2` | AND Vx, Vy | VX &= VY |
| `8XY3` | XOR Vx, Vy | VX ^= VY |
| `8XY4` | ADD Vx, Vy | VX += VY; VF = 1 if overflow, else 0 |
| `8XY5` | SUB Vx, Vy | VX -= VY; VF = 1 if VX > VY (no borrow), else 0 |
| `8XY6` | SHR Vx | VX >>= 1; VF = shifted-out bit |
| `8XY7` | SUBN Vx, Vy | VX = VY - VX; VF = 1 if VY > VX, else 0 |
| `8XYE` | SHL Vx | VX <<= 1; VF = shifted-out bit |

### Memory
| Opcode | Mnemonic | Description |
|--------|----------|-------------|
| `ANNN` | LD I, addr | I = NNN |
| `FX1E` | ADD I, Vx | I += VX |
| `FX29` | LD F, Vx | I = address of font sprite for digit VX |
| `FX33` | LD B, Vx | Store BCD of VX at I, I+1, I+2 |
| `FX55` | LD [I], Vx | Store V0–VX at memory starting at I |
| `FX65` | LD Vx, [I] | Load V0–VX from memory starting at I |

### Display
| Opcode | Mnemonic | Description |
|--------|----------|-------------|
| `DXYN` | DRW Vx, Vy, n | Draw N-byte sprite at (VX, VY); VF = collision |

### Keyboard
| Opcode | Mnemonic | Description |
|--------|----------|-------------|
| `EX9E` | SKP Vx | Skip if key VX is pressed |
| `EXA1` | SKNP Vx | Skip if key VX is NOT pressed |
| `FX0A` | LD Vx, K | Block until key press, store key in VX |

### Timers
| Opcode | Mnemonic | Description |
|--------|----------|-------------|
| `FX07` | LD Vx, DT | VX = delay timer |
| `FX15` | LD DT, Vx | delay timer = VX |
| `FX18` | LD ST, Vx | sound timer = VX |

---

## Implementation Quirks

Different CHIP-8 implementations have historical inconsistencies:

### Shift Instructions (8XY6, 8XYE)
- **Original COSMAC VIP**: VX = VY >> 1 or VX = VY << 1 (copies VY first)
- **Modern/CHIP-48/SCHIP**: VX = VX >> 1 or VX = VX << 1 (ignores VY)
- **Recommendation**: Make configurable, default to modern behavior

### Jump with Offset (BNNN)
- **Original**: PC = NNN + V0
- **CHIP-48/SCHIP**: PC = XNN + VX (uses X from opcode)
- **Recommendation**: Use original V0 behavior

### Memory Store/Load (FX55, FX65)
- **Original**: I is incremented after each byte (I = I + X + 1)
- **Modern**: I is unchanged
- **Recommendation**: Make configurable, default to modern behavior

### Index Overflow (FX1E)
- **Original Amiga interpreter**: Sets VF = 1 if I + VX > 0xFFF
- **Most implementations**: VF unchanged
- **Recommendation**: Don't set VF (most compatible)

### Display Wait
- **Original**: DXYN waits for vertical blank
- **Modern**: Usually not implemented
- **Recommendation**: Optional, can help with flickering games

### Execution Speed
- **Typical**: ~700 instructions per second
- **Recommendation**: Make configurable (500–1000 Hz range)

---

## Crossterm Crate Reference

**Crate**: `crossterm` (v0.29.0+)
**Purpose**: Cross-platform terminal manipulation (cursor, input, colors, screen)

### Cargo.toml
```toml
[dependencies]
crossterm = "0.29"
```

### Key Modules

| Module | Purpose |
|--------|---------|
| `cursor` | Move, show/hide, save/restore cursor position |
| `event` | Read keyboard/mouse events |
| `style` | Colors and text attributes |
| `terminal` | Raw mode, alternate screen, clear, size |

### Raw Mode
Required for real-time input without line buffering:
```rust
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

enable_raw_mode()?;
// ... your game loop ...
disable_raw_mode()?;
```

### Event Handling
```rust
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::time::Duration;

// Non-blocking poll
if event::poll(Duration::from_millis(16))? {
    if let Event::Key(KeyEvent { code, .. }) = event::read()? {
        match code {
            KeyCode::Char('q') => break,
            KeyCode::Char('w') => { /* handle W key */ }
            KeyCode::Esc => break,
            _ => {}
        }
    }
}
```

### Cursor & Drawing
```rust
use crossterm::{
    cursor::{Hide, Show, MoveTo},
    execute,
    style::Print,
    terminal::{Clear, ClearType},
};
use std::io::stdout;

let mut stdout = stdout();

// Hide cursor, clear screen, draw
execute!(stdout, Hide, Clear(ClearType::All))?;
execute!(stdout, MoveTo(x, y), Print("█"))?;

// Show cursor on exit
execute!(stdout, Show)?;
```

### Alternate Screen
Preserves original terminal content:
```rust
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};

execute!(stdout, EnterAlternateScreen)?;
// ... run emulator ...
execute!(stdout, LeaveAlternateScreen)?;
```

### Performance Tips
- Use `queue!` macro with explicit `flush()` for batched operations
- Poll at 60 Hz for smooth input (matches timer rate)

---

## SDL2 Crate Reference

**Crate**: `sdl2` (v0.38.0+)
**Purpose**: Graphics, audio, and input via SDL2 library bindings

### Cargo.toml
```toml
[dependencies]
sdl2 = "0.38"

# Or with bundled SDL2 (easier setup, compiles SDL2 from source):
# sdl2 = { version = "0.38", features = ["bundled"] }
```

### Features
- `bundled` - Compile SDL2 from source (recommended for easy setup)
- `image` - SDL2_image for loading PNG/JPG
- `mixer` - SDL2_mixer for audio
- `ttf` - SDL2_ttf for fonts

### Basic Window Setup
```rust
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // CHIP-8: 64x32 scaled up (e.g., 10x = 640x320)
    let window = video_subsystem.window("CHIP-8", 640, 320)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }

        // Update and render
        canvas.present();
    }

    Ok(())
}
```

### Drawing Pixels (for CHIP-8 Display)
```rust
use sdl2::rect::Rect;

const SCALE: u32 = 10;

fn draw_display(canvas: &mut Canvas<Window>, display: &[[bool; 64]; 32]) {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();

    canvas.set_draw_color(Color::WHITE);
    for y in 0..32 {
        for x in 0..64 {
            if display[y][x] {
                let rect = Rect::new(
                    (x as u32 * SCALE) as i32,
                    (y as u32 * SCALE) as i32,
                    SCALE,
                    SCALE,
                );
                canvas.fill_rect(rect).unwrap();
            }
        }
    }
    canvas.present();
}
```

### Keyboard Input
```rust
use sdl2::keyboard::Keycode;

// Map modern keyboard to CHIP-8 hex keypad
fn keycode_to_chip8(keycode: Keycode) -> Option<u8> {
    match keycode {
        Keycode::Num1 => Some(0x1), Keycode::Num2 => Some(0x2),
        Keycode::Num3 => Some(0x3), Keycode::Num4 => Some(0xC),
        Keycode::Q => Some(0x4),    Keycode::W => Some(0x5),
        Keycode::E => Some(0x6),    Keycode::R => Some(0xD),
        Keycode::A => Some(0x7),    Keycode::S => Some(0x8),
        Keycode::D => Some(0x9),    Keycode::F => Some(0xE),
        Keycode::Z => Some(0xA),    Keycode::X => Some(0x0),
        Keycode::C => Some(0xB),    Keycode::V => Some(0xF),
        _ => None,
    }
}
```

### Audio (Beep for Sound Timer)
```rust
use sdl2::audio::{AudioCallback, AudioSpecDesired};

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for x in out.iter_mut() {
            *x = if self.phase < 0.5 { self.volume } else { -self.volume };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

// Setup audio device
let audio_subsystem = sdl_context.audio()?;
let spec = AudioSpecDesired {
    freq: Some(44100),
    channels: Some(1),
    samples: None,
};
let device = audio_subsystem.open_playback(None, &spec, |spec| {
    SquareWave {
        phase_inc: 440.0 / spec.freq as f32,
        phase: 0.0,
        volume: 0.25,
    }
})?;

// Play when sound_timer > 0
device.resume();  // Start playing
device.pause();   // Stop playing
```

---

## Recommended Project Structure

```
chip8/
├── Cargo.toml
├── CLAUDE.md              # This file
├── src/
│   ├── main.rs            # Entry point, argument parsing
│   ├── chip.rs            # Main Chip8 struct, fetch-decode-execute
│   ├── memory.rs          # 4KB RAM + font loading
│   ├── register.rs        # V0-VF, I register
│   ├── stack.rs           # 16-level stack
│   ├── program_counter.rs # PC management
│   ├── timer.rs           # Delay and sound timers
│   ├── display.rs         # 64x32 framebuffer
│   ├── keypad.rs          # 16-key input state
│   └── frontend/
│       ├── mod.rs
│       ├── terminal.rs    # Crossterm-based UI
│       └── sdl.rs         # SDL2-based UI
└── roms/                  # Test ROMs
```

---

## Testing Resources

### Test ROMs
- **chip8-test-suite** by Timendus: Comprehensive opcode tests
- **BC_test.ch8**: Basic test ROM
- **test_opcode.ch8**: Opcode coverage test
- **PONG**, **TETRIS**, **INVADERS**: Classic games for playability testing

### Useful Links
- [Tobias V. Langhoff's Guide](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/)
- [Cowgod's Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
- [Awesome CHIP-8](https://chip-8.github.io/links/)
- [CHIP-8 Test Suite](https://github.com/Timendus/chip8-test-suite)

---

## Implementation Checklist

- [ ] Memory with font loaded
- [ ] All 16 V registers + I register
- [ ] 16-level stack
- [ ] Program counter with fetch logic
- [ ] Instruction decoder
- [ ] All 35 opcodes
- [ ] 64x32 display buffer
- [ ] Sprite drawing with collision detection
- [ ] 16-key input handling
- [ ] 60 Hz delay timer
- [ ] 60 Hz sound timer with audio output
- [ ] ROM loading
- [ ] Configurable quirks mode
- [ ] Frontend (terminal or SDL2)
