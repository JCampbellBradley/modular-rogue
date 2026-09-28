# Initiative

The game loop should keep track of a priority queue of entities, whose turns are taken in that order, before being added back to the tail of the queue. Entities whose initiative is tied should take their turns simultaneously and to an extent asynchronously.

# Turn Phases

## Phase 1: Decision

First, entities should decide how to take their turn.