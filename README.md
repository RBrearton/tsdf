# Time series data format (tsdf)

Welcome to the home of the time series data format. This data format was heavily inspired by the
excellent hdf5 format. The time series data format is not intended to be a reimplementation of the
hdf5 spec, which is long and highly technical. Instead, it is supposed to be a simple implementation
of a file format that allows readers to read while a writer writes, mirroring hdf5's single writer
multiple reader (swmr) mode.

Unlike hdf5's swmr mode, tsdf, by default, allows arbitrary additions (including new directories,
new metadata tags and new arrays) in swmr mode. This was, in fact, the main driver behind developing
the tsdf format: to allow (additive) mutations to the file hierarchy, as well as allow data to be
added to arrays, with a single writer and multiple readers.

# Roadmap

To achieve any level of success, various languages need bindings to this library. The current
intention is to build python bindings first, followed by C# bindings.

Once bindings for python and C# are complete, tested and working, a web UI for monitoring arrays in
tsdf files will be built. After all, this file format was designed from the ground up to enhance
interactions with time series data.

The intention is to build a web UI that takes full advantage of the features of the tsdf, with a
configurable dashboard, ability to add metadata to the file from within the dashboard, and a
plotting engine for visualizing:

- 1D array data as a function of time (for 1D arrays)
- A full suite for working with 2D (image) time series data sets, including:
  - Region of interest selection (and plots of integrated roi signal as a function of time)
  - Background subtraction (via region of interests)
- Filtering options for all plots
- The option to write filtered/background subtracted data back to the tsdf file in real time.
