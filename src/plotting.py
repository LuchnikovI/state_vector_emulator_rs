import numpy as np
import matplotlib.pyplot as plt
import imageio.v2 as imageio
import os

path = os.path.join(os.path.dirname(__file__), os.path.normpath('../target/release/sigmaz_dynamics.txt'))
data = np.genfromtxt(path, delimiter=',')[:-1].reshape((-1, 5, 5))
filenames = []
for i, slice in enumerate(data):
  plt.figure()
  im = plt.imshow(slice, cmap="inferno", vmax=1, vmin=-1)
  plt.xlabel('x')
  plt.ylabel('y')
  plt.colorbar(im)
  plt.savefig("{}_slice.png".format(i))
  filenames.append("{}_slice.png".format(i))
  plt.close()

with imageio.get_writer('sigmaz_dynamics.gif', mode='I') as writer:
    for filename in filenames:
        image = imageio.imread(filename)
        writer.append_data(image)

for filename in set(filenames):
    os.remove(filename)