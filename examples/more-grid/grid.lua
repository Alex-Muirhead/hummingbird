-- a simple grid

width = 15;
height = 15;
resolution = 2;

a = Vector3:new{x=0.0, y=0.0};
b = Vector3:new{x=width, y=0.0};
c = Vector3:new{x=width, y=height};
d = Vector3:new{x=0.0, y=height};

cluster_east = GaussianFunction:new{m=0.3, s=0.3, ratio=0.1}
cluster_west = GaussianFunction:new{m=0.3, s=0.3, ratio=0.1}
clusterlist = {north=none, south=none, east=none, west=none}

patch = CoonsPatch:new{p00=a, p10=b, p11=c, p01=d};
grid = StructuredGrid:new{
   psurface=patch, niv=resolution*width+1, njv=resolution*height+1, cfList=clusterlist
};

registerFluidGrid{grid=grid, fsTag="initial"};
