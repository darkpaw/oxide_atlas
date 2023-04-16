Install dependencies on Ubuntu 22.04:

sudo apt install libproj-dev libclang-dev

https://docs.ogc.org/is/13-082r2/13-082r2.html#1

https://enonline.supermap.com/iExpress9D/API/WMTS/wmts_introduce.htm

```xml

         <TileMatrix>
            <ows:Identifier>8</ows:Identifier>
            <ScaleDenominator>2183915.093862179</ScaleDenominator>
            <TopLeftCorner>
               -20037508.3427892 20037508.3427892
            </TopLeftCorner>
            <TileWidth>256</TileWidth>
            <TileHeight>256</TileHeight>
            <MatrixWidth>256</MatrixWidth>
            <MatrixHeight>256</MatrixHeight>
         </TileMatrix>

```

https://github.com/Logicalshift/flo_draw

```css
body {
    margin: 40px auto;
    max-width: 960px;
    line-height: 1.6;
    font-size: 22px;
    color: #444;
    background-color: #EEE;
    padding: 0 16px
}

h1, h2, h3 {
    line-height: 1.25
}
```

https://portal.ogc.org/files/?artifact_id=33269


https://example.com/gwc/service/wmts?layer=workspace_name:layer_name&tilematrixset=EPSG%3A900913&Service=WMTS&Request=GetTile&Version=1.0.0&Format=image%2Fpng&TileMatrix=EPSG%3A900913%3A14&TileCol=15083&TileRow=9778


To test the server, run the Rust program and make a GET request to the following URL:

http://localhost:8080/tiles?tile_matrix=EPSG%3A900913%3A14&tile_col=15083&tile_row=9778

You should receive a response containing the parsed URL parameters:

TileMatrix: EPSG:900913:14
Projection: 900913
Zoom: 14
TileCol: 15083
TileRow: 9778


http://localhost:8080/tiles?tile_matrix=EPSG%3A900913%3A13&tile_col=15083&tile_row=9778

http://localhost:8080/map


[YYMMDD:HHMMSSmmm] Z 8 X 235 Y 157 P 900913
Tile corner coordinates: Top-Left (16750104.630300347, 44614764.669491574), Bottom-Right (16906647.664228387, 44771307.70341962)

[YYMMDD:HHMMSSmmm] Z 8 X 238 Y 156 P 900913
Tile corner coordinates: Top-Left (17219733.73208447, -4383204.949985139), Bottom-Right (17376276.766012512, -4539747.9839131795)

https://github.com/flatgeobuf/flatgeobuf

https://gdal.org/drivers/vector/flatgeobuf.html

https://www.kaggle.com/datasets/juanmah/world-cities?resource=download