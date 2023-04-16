import pandas as pd
import geopandas as gpd
from pyproj import Transformer
from shapely.geometry import Point

# Load the CSV file into a DataFrame
csv_file = "../data/worldcities.csv"
df = pd.read_csv(csv_file)

# Transform coordinates from EPSG:4326 to EPSG:3857
transformer = Transformer.from_crs("EPSG:4326", "EPSG:3857", always_xy=True)
df["lng"], df["lat"] = transformer.transform(df["lng"].values, df["lat"].values)

# Convert DataFrame to GeoDataFrame
geometry = [Point(xy) for xy in zip(df["lng"], df["lat"])]
gdf = gpd.GeoDataFrame(df, crs="EPSG:3857", geometry=geometry)

# Write the GeoDataFrame to a FlatGeoBuf file
fgb_file = "worldcities_transformed.fgb"
gdf.to_file(fgb_file, driver="FlatGeobuf")

print(f"Transformed data saved to '{fgb_file}'.")
