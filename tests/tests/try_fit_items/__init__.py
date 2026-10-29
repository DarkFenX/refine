"""
Tests here largely replicate validation tests. They earn their place mostly because of some
optimizations in try-fit-items, which duplicate some checks to run them faster than regular
validations do (in a static way, without actually adding items to fit).
"""
