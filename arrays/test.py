#! env python

import numpy as np
import rust_module

rust_module.rust_function([1, 2, 3, 4, 5])
rust_module.rust_function(np.zeros(5, dtype=np.int8))
