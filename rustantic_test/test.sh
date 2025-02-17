#!/bin/bash

uv pip install .
maturin develop 

# Run the tests 
pytest tests
