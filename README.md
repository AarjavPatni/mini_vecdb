# Design Decisions

1. Hardcoded the length of each embedding to be 10 for testing purposes only since this is just a
   toy project.
2. Used `SVector` from the `nalgebra` library to be able to leverage the `dot()` method.
3. I'm sure there's a method to find the cosine similarity of two SVectors but doing it myself was
   an excuse to get more familiar with the library.

# Distance Algorithm

- I implemented the Cosine distance algorithm using the advice from [this](https://www.singlestore.com/blog/distance-metrics-in-machine-learning-simplfied/#how-vector-similarity-search-is-calculated) blog post by SingleStore, a well-known
data platform.
- It mentions that the Cosine algorithm is "particularly useful in fields like text analysis and information retrieval", which is integral to search engines.

# Improvements

1. Need to find a better way to find top-k elements. This is a quite inefficient.
2. Wrap the db under a struct and use `impl` instead.
3. Write comprehensive tests.

# Future Goals

1. Write the dot() function by myself.

