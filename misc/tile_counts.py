def tiles_per_zoom_level(max_zoom=18):
    running_total = 0
    zoom_levels = {}

    for _zoom in range(max_zoom + 1):
        tiles = 2 ** (_zoom * 2)
        running_total += tiles
        zoom_levels[_zoom] = {"tiles": tiles, "running_total": running_total}

    return zoom_levels

# Example usage:
result = tiles_per_zoom_level()
for _zoom, data in result.items():
    print(f"Zoom level: {_zoom} - Tiles: {data['tiles']} - Running total: {data['running_total']}")


bits_per_channel = 8
channels = 4
pixels_per_tile = 256 * 256

bits_per_tile = pixels_per_tile * channels * bits_per_channel
bytes_per_tile = bits_per_tile // 8

print(f"Bytes per tile: {bytes_per_tile}")

# Example usage:
result = tiles_per_zoom_level()
for _zoom, data in result.items():
    print(f"Zoom level: {_zoom} - Tiles: {data['tiles']} - Running total: {data['running_total']}")

    total = data['running_total']
    total_bytes = total * bytes_per_tile
    print(f"GB {total_bytes / 1024 / 1024 / 1024}")

