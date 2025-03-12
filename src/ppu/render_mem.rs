use macroquad::color::BLACK;
use xf::{
    mq::draw::draw_rect,
    num::{
        irect::rect,
        ivec2::{i2, IVec2},
    },
};

use crate::{
    consts::{P4, P8},
    mem::{cram::Cram, io_regs::IoReg, sections::MemSection, Addr},
    ppu::render_util::draw_pixel_c_bg,
    sys::Sys,
    util::bits::Bits,
};

use super::{
    attrs::BgAttrs,
    consts::*,
    palette::Palette,
    render_util::{draw_line, draw_pixel, tile_data_idx_to_addr},
};

/// Renders one of the tile data blocks to the screen.
#[inline]
pub fn render_tile_data_block(sys: &Sys, block_addr: Addr, vram_bank: usize, org: IVec2) {
    let mut i = 0;
    let range = block_addr..(block_addr + TILE_DATA_BLOCK_SIZE);
    for addr in range.clone().step_by(16) {
        let x = i % TILE_DATA_P8_SIZE.x;
        let y = i / TILE_DATA_P8_SIZE.x;

        let rel_addr = (addr - TILE_DATA_ADDR_8000) as usize;
        let tile_size = TILE_DATA_TILE_SIZE as usize;
        //let bytes = &sys.mem.vram.as_slice()[rel_addr..(rel_addr + tile_size)];
        let bytes = sys
            .mem
            .vram
            .get_range(vram_bank, rel_addr..(rel_addr + tile_size));

        i += 1;

        draw_tile(sys, bytes, &None, org + i2(x * 8, y * 8));
    }
}

/// Renders the entire tilemap, starting at `tile_map_addr`, to the screen .
#[inline]
pub fn render_tile_map(sys: &Sys, tile_map_addr: Addr, org: IVec2) {
    for i in 0..TILE_MAP_P8_SIZE.product() {
        let x = i % TILE_MAP_P8_SIZE.x;
        let y = i / TILE_MAP_P8_SIZE.x;
        let addr = (i as u16) + tile_map_addr;

        draw_tile_from_map(sys, i2(x, y), addr, org);
    }
}

/// Renders the viewport bounds detrmined by the SCX/SCY registers.
#[inline]
pub fn render_scroll_view_area(sys: &Sys, org: IVec2) {
    let scx = sys.mem.io_regs.get(IoReg::Scx) as i32;
    let scy = sys.mem.io_regs.get(IoReg::Scy) as i32;

    let end_x = (scx + VIEWPORT_SIZE.x) % 256;
    let end_y = (scy + VIEWPORT_SIZE.y) % 256;

    let x_wraps = scx > end_x;
    let y_wraps = scy > end_y;

    if !x_wraps {
        draw_line(org + i2(scx, scy), VIEWPORT_SIZE.x, false, BLACK);
        draw_line(org + i2(scx, end_y), VIEWPORT_SIZE.x, false, BLACK);
    } else {
        draw_line(org + i2(scx, scy), 255 - scx, false, BLACK);
        draw_line(org + i2(0, scy), end_x, false, BLACK);

        draw_line(org + i2(scx, end_y), 255 - scx, false, BLACK);
        draw_line(org + i2(0, end_y), end_x, false, BLACK);
    }

    if !y_wraps {
        draw_line(org + i2(scx, scy), VIEWPORT_SIZE.y, true, BLACK);
        draw_line(org + i2(end_x, scy), VIEWPORT_SIZE.y, true, BLACK);
    } else {
        draw_line(org + i2(scx, scy), 255 - scy, true, BLACK);
        draw_line(org + i2(scx, 0), end_y, true, BLACK);

        draw_line(org + i2(end_x, scy), 255 - scy, true, BLACK);
        draw_line(org + i2(end_x, 0), end_y, true, BLACK);
    }
}

#[inline]
fn draw_tile_from_map(sys: &Sys, pos: IVec2, map_addr: Addr, org: IVec2) {
    let lcdc = sys.mem.io_regs.get(IoReg::Lcdc);
    let is_mode_8000 = lcdc.bit(4) == 1;
    let data_idx = sys.mem.vram.get(0, map_addr);
    let attrs = if sys.is_cgb_mode() {
        Some(BgAttrs::new(sys, map_addr))
    } else {
        None
    };

    let data_addr = tile_data_idx_to_addr(data_idx as u16, is_mode_8000);

    let addr = (data_addr - MemSection::Vram.start_addr()) as usize;
    let vram_bank = attrs.as_ref().map(|a| a.bank).unwrap_or(0);
    let bytes = sys.mem.vram.get_range(vram_bank, addr..(addr + 16));

    let org = pos * P8 + org;
    draw_tile(sys, bytes, &attrs, org);
}

#[inline]
fn draw_tile(sys: &Sys, bytes: &[u8], attrs: &Option<BgAttrs>, org: IVec2) {
    const PALETTE: Palette = Palette::default();

    let flip_x = attrs.as_ref().map(|a| a.x_flip).unwrap_or(false);
    let flip_y = attrs.as_ref().map(|a| a.y_flip).unwrap_or(false);
    let color_palette = attrs.as_ref().map(|a| a.color_palette).unwrap_or(0);

    for pos in rect(0, 0, 8, 8).iter() {
        let pos_y = if flip_y { 7 - pos.y } else { pos.y };
        let idx = (pos_y * 2) as usize;
        let bit = if flip_x { pos.x } else { 7 - pos.x };
        let lower = bytes[idx].bit(bit as u8);
        let upper = bytes[idx + 1].bit(bit as u8);

        let color_id = (upper << 1) | lower;

        if attrs.is_none() {
            draw_pixel::<false>(pos + org, &PALETTE, color_id);
        } else {
            draw_pixel_c_bg(sys, pos + org, color_palette, color_id);
        }
    }
}

#[inline]
pub fn draw_palettes(sys: &Sys, org: IVec2) {
    draw_cram_palettes(sys, sys.mem.io_regs.bg_cram(), org);
    draw_cram_palettes(sys, sys.mem.io_regs.obj_cram(), org + (i2(10, 0) * P4));
}

#[inline]
pub fn draw_cram_palettes(sys: &Sys, cram: &Cram, org: IVec2) {
    for i in 0..32 {
        let palette_id = i / 4;
        let color_id = i % 4;

        let color_idx = cram.get(palette_id, color_id);
        let color = sys.ppu.colors().get(color_idx);

        const SIZE: i32 = 4;
        let bounds =
            rect(palette_id as i32 * SIZE, color_id as i32 * SIZE, SIZE, SIZE).offset_by(org);

        draw_rect(bounds, color);
    }
}
