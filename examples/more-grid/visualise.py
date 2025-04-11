import pyvista as pv

filename = "./lmrsim/vtk/fluid.pvd"
reader = pv.PVDReader(filename)
# Get the final time-point
reader.set_active_time_point(-1)

file = reader.read()

plotter = pv.Plotter()
_ = plotter.add_mesh(file, scalars="Qrad", label="radiation")

# Must add data before changing camera
plotter.camera.tight(padding=0.05, view="xy")  # Enforces Parallel Projection too
plotter.show()
