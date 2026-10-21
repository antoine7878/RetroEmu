use super::constants::{
    BGP, DMA, LCDC, LCDC_OFF, LY, LYC, OAM_END, OAM_SIZE, OAM_START, OBP0, OBP1, SCX, SCY, STAT, VBK, VRAM_END,
    VRAM_SIZE, VRAM_START, WX, WY,
};
use super::io::{IOReg, IORegister};
use super::{HEIGHT, PALETTE_ORIGNAL, WIDTH};

#[derive(PartialEq, Clone, Eq)]
pub enum PpuMode {
    HBlank = 0,
    VBlank = 1,
    OamScan = 2,
    Drawing = 3,
}

pub struct Ppu {
    pixels: [[[u8; 3]; WIDTH]; HEIGHT],
    redraw: bool,
    dot: u16,

    stat_line: bool,
    mode: PpuMode,
    window_line: u16,

    selected_objects: [[u8; 4]; 10],
    selected_objects_len: usize,

    vram: [[u8; VRAM_SIZE]; 2],
    oam: [u8; OAM_SIZE],
    lcdc: IORegister<0xFF>,
    stat: IORegister<0x7F>,
    scy: IORegister<0xFF>,
    scx: IORegister<0xFF>,
    ly: IORegister<0xFF>,
    lyc: IORegister<0xFF>,
    dma: IORegister<0xFF>,
    bgp: IORegister<0xFF>,
    opb0: IORegister<0xFF>,
    opb1: IORegister<0xFF>,
    wy: IORegister<0xFF>,
    wx: IORegister<0xFF>,
    vbk: IORegister<0x01>,
}

impl Default for Ppu {
    fn default() -> Self {
        Self {
            pixels: [[[0; 3]; WIDTH]; HEIGHT],
            dot: 0,
            vram: [[0; VRAM_SIZE]; 2],
            oam: [0; OAM_SIZE],
            selected_objects: [[0; 4]; 10],
            selected_objects_len: 0,
            stat_line: false,
            redraw: false,
            window_line: 0,
            lcdc: IORegister::default(),
            stat: IORegister::default(),
            scy: IORegister::default(),
            scx: IORegister::default(),
            ly: IORegister::default(),
            lyc: IORegister::default(),
            dma: IORegister::default(),
            bgp: IORegister::default(),
            opb0: IORegister::default(),
            opb1: IORegister::default(),
            wy: IORegister::default(),
            wx: IORegister::default(),
            vbk: IORegister::default(),
            mode: PpuMode::HBlank,
        }
    }
}

impl Ppu {
    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            VRAM_START..VRAM_END => self.read_vram(addr),
            OAM_START..OAM_END => self.read_oam(addr),
            LCDC => self.lcdc.read(),
            STAT => self.stat.read(),
            SCY => self.scy.read(),
            SCX => self.scx.read(),
            LY => self.ly.read(),
            LYC => self.lyc.read(),
            DMA => self.dma.read(),
            BGP => self.bgp.read(),
            OBP0 => self.opb0.read(),
            OBP1 => self.opb1.read(),
            WY => self.wy.read(),
            WX => self.wx.read(),
            VBK => self.vbk.read(),
            _ => panic!("Address not in PPU"),
        }
    }

    pub fn read_vram(&self, addr: u16) -> u8 {
        match self.mode {
            PpuMode::Drawing => 0xFF,
            _ => self.vram[self.vbk.read() as usize][(addr - VRAM_START) as usize],
        }
    }

    pub fn read_oam(&self, addr: u16) -> u8 {
        match self.mode {
            PpuMode::OamScan | PpuMode::Drawing => 0xFF,
            _ => self.oam[(addr - OAM_START) as usize],
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            VRAM_START..VRAM_END => self.write_vram(addr, value),
            OAM_START..OAM_END => self.write_oam(addr, value),
            LCDC => self.lcdc.write(value),
            STAT => self.stat.write(value),
            SCY => self.scy.write(value),
            SCX => self.scx.write(value),
            LY => self.ly.write(value),
            LYC => self.lyc.write(value),
            DMA => self.dma.write(value),
            BGP => self.bgp.write(value),
            OBP0 => self.opb0.write(value),
            OBP1 => self.opb1.write(value),
            WY => self.wy.write(value),
            WX => self.wx.write(value),
            VBK => self.vbk.write(value),
            _ => panic!("Address not in PPU"),
        }
    }

    pub fn write_vram(&mut self, addr: u16, value: u8) {
        match self.mode {
            PpuMode::Drawing => (),
            _ => self.vram[self.vbk.read() as usize][(addr - VRAM_START) as usize] = value,
        }
    }

    pub fn write_oam(&mut self, addr: u16, value: u8) {
        match self.mode {
            PpuMode::OamScan | PpuMode::Drawing => (),
            _ => self.oam[(addr - OAM_START) as usize] = value,
        }
    }

    fn tile_offset(&self, id: u8) -> usize {
        let unsigned = matches!(self.mode, _) || self.lcdc.read() & 0x10 != 0;
        if unsigned { id as usize * 16 } else { (0x1000 + (id as i8 as i32) * 16) as usize }
    }

    fn get_tile(&self, id: u8) -> &[u8; 16] {
        let start = self.tile_offset(id);
        self.vram[0][start..start + 16].try_into().unwrap()
    }

    fn set_mode<const T: u8>(&mut self, mode: PpuMode, ifl: &mut IORegister<T>) {
        self.mode = mode;
        self.update_stat_line(ifl);
    }

    fn update_stat_line<const T: u8>(&mut self, ifl: &mut IORegister<T>) {
        let lyc_match = self.ly == self.lyc;

        let stat = self.stat.read();
        let line = (stat & 0x08 != 0 && self.mode == PpuMode::HBlank)
            || (stat & 0x10 != 0 && self.mode == PpuMode::VBlank)
            || (stat & 0x20 != 0 && self.mode == PpuMode::OamScan)
            || (stat & 0x40 != 0 && lyc_match);

        if line && !self.stat_line {
            ifl.set(1);
        }
        self.stat_line = line;
    }

    // pub fn read_stat(&self) -> u8 { &self.stat | ((self.ly == self.lyc) as u8) << 2 | self.mode.clone() as u8 | 0x80 }

    fn tick<const T: u8>(&mut self, ifl: &mut IORegister<T>) {
        self.dot += 1;
        match self.mode {
            PpuMode::OamScan if self.dot == 80 => {
                self.scan_oam();
                self.set_mode(PpuMode::Drawing, ifl);
            }
            PpuMode::Drawing if self.dot == 252 => {
                self.render_scanline();
                self.set_mode(PpuMode::HBlank, ifl);
            }
            PpuMode::HBlank | PpuMode::VBlank if self.dot == 456 => {
                self.dot = 0;
                self.next_line(ifl);
            }
            _ => {}
        }
        self.update_stat_line(ifl);
    }

    fn scan_oam(&mut self) {
        let line = self.ly.read() as i16 + 16;
        let height: i16 = if self.lcdc.is_set(2) { 16 } else { 8 };

        self.selected_objects_len = 1;
        for o in self.oam.as_chunks::<4>().0 {
            let y = o[0] as i16;
            if y <= line && line < y + height {
                self.selected_objects[self.selected_objects_len] = [o[0], o[1], o[2], o[3]];
                self.selected_objects_len += 1;
                if self.selected_objects_len == 10 {
                    break;
                }
            }
        }
    }

    fn render_scanline(&mut self) {
        // write row `ly` of self.pixels
        let mut line_buffer = [0u8; WIDTH];
        self.render_background(&mut line_buffer);
        self.render_window(&mut line_buffer);
        self.render_objects(&mut line_buffer);
    }

    fn render_background(&mut self, line_buffer: &mut [u8; WIDTH]) {
        let screen_y = self.ly.read();
        let pixel_row = &mut self.pixels[screen_y as usize];

        if self.lcdc.is_clear(0) {
            pixel_row.iter_mut().for_each(|pixel| *pixel = PALETTE_ORIGNAL[0]);
            return;
        }

        let tile_map_base: u16 = if self.lcdc.is_clear(3) { 0x9800 } else { 0x9C00 };
        let scroll_x = self.scx.read();
        let scroll_y = self.scy.read();
        let bg_palette = self.bgp.read();

        let map_y = scroll_y.wrapping_add(screen_y);
        let tile_row = (map_y / 8) as u16;
        let row_in_tile = (map_y % 8) as usize;

        for (screen_x, line_pixel) in line_buffer.iter_mut().enumerate() {
            let map_x = scroll_x.wrapping_add(screen_x as u8);
            let tile_col = (map_x / 8) as u16;

            let tile_id = self.read(tile_map_base + tile_row * 32 + tile_col);
            // do it once every : x % 8 == 0
            let tile = self.get_tile(tile_id);
            let low_byte = tile[row_in_tile * 2];
            let high_byte = tile[row_in_tile * 2 + 1];

            let bit_index = 7 - (map_x % 8);
            let color_id = (((high_byte >> bit_index) & 1) << 1) | ((low_byte >> bit_index) & 1);
            let shade = (bg_palette >> (color_id * 2)) & 0b11;

            *line_pixel = shade;
        }
    }

    fn render_window(&mut self, line_buffer: &mut [u8; WIDTH]) {
        // if self.lcdc.is_clear(0) {
        //     return;
        // }
        // let bg_map = if self.lcdc.is_clear(6) { 0x9800 } else { 0x9C00 };
        // for i in (0..WIDTH).step_by(8) {}
        for (x, lx) in line_buffer.iter_mut().enumerate() {}
    }

    fn render_objects(&mut self, _line_buffer: &mut [u8; WIDTH]) {
        if self.lcdc.is_set(1) {
            return;
        }
        for [y, x, id, attr] in self.selected_objects[0..self.selected_objects_len].iter() {
            let tile = self.get_tile(*id);
            let ly = self.ly.read();
        }
    }

    fn next_line<const T: u8>(&mut self, ifl: &mut IORegister<T>) {
        self.ly += 1;
        match self.ly.read() {
            144 => {
                self.set_mode(PpuMode::VBlank, ifl);
                ifl.set(0);
                self.redraw = true;
            }
            154 => {
                self.ly.write(0);
                self.window_line = 0;
                self.set_mode(PpuMode::OamScan, ifl);
            }
            0..144 => self.set_mode(PpuMode::OamScan, ifl),
            _ => (),
        }
    }

    pub fn ticks<const T: u8>(&mut self, dots: u8, ifl: &mut IORegister<T>) {
        if self.lcdc.is_clear(LCDC_OFF) {
            self.dot = 0;
            self.ly.write(0);
            return;
        }
        for _ in 0..dots {
            self.tick(ifl);
        }
    }

    pub fn take_redraw(&mut self) -> bool { std::mem::take(&mut self.redraw) }
    pub const fn get_pixels(&self) -> &[u8] { self.pixels.as_flattened().as_flattened() }
}
