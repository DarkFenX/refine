Stages are just a short description of how much data was available for the mutation:

- stage 1: mutator is not available;
- stage 2: mutator is available, but mutated item ID cannot be found for the base item ID;
- stage 3: mutator is available, mutated item ID is located for the base item ID, but mutated item itself is not available;
- stage 4: all the data was available.

Mutation is dormant on stages 1 and 2, and effective on stage 4.

Stage 3 does not reach runtime: data cleanup drops mutated item mappings whose base, mutator or mutated item is missing, along with mutators no other mapping keeps, so such data becomes stage 1, or stage 2 when another mapping keeps the mutator.
