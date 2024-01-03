package main

import (
	"fmt"
	"math"
	"math/bits"
	"slices"

	"github.com/tmw/pathfind"
)

const (
	SampleSeed = 10
	ActualSeed = 1364
)

var floor = make(map[string]bool)

type Coordinate struct {
	x, y int
}

// return manhattan distance between two coordinates
func (c Coordinate) Dist(t Coordinate) int {
	return int(math.Abs(float64(c.y-t.y)) + math.Abs(float64(c.x-t.x)))
}

func (c Coordinate) Neighbours() []Coordinate {
	n := make([]Coordinate, 4)
	n = append(n, Coordinate{c.x + 1, c.y})
	n = append(n, Coordinate{c.x, c.y + 1})

	if c.x > 0 {
		n = append(n, Coordinate{c.x - 1, c.y})
	}

	if c.y > 0 {
		n = append(n, Coordinate{c.x, c.y - 1})
	}

	return n
}

func partOne() {
	var (
		start  = Coordinate{1, 1}
		finish = Coordinate{31, 39}
	)

	f := pathfind.NewAStar[Coordinate](start, &pathfind.FuncAdapter[Coordinate]{
		NeighboursFn: func(c Coordinate) []Coordinate {
			n := c.Neighbours()
			return slices.DeleteFunc(n, func(c Coordinate) bool {
				return !isWalkable(c.x, c.y, ActualSeed)
			})
		},

		CostToFinishFn: func(c Coordinate) int {
			return c.Dist(finish)
		},

		IsFinishFn: func(c Coordinate) bool {
			return c == finish
		},
	})

	path := f.Walk()
	fmt.Printf("answer part one = %d\n", len(path))
}

func partTwo() {
	var (
		start   = Coordinate{1, 1}
		finish  = Coordinate{31, 39}
		visited = make(map[Coordinate]struct{})
	)

	f := pathfind.NewBFS[Coordinate](start, &pathfind.FuncAdapter[Coordinate]{
		NeighboursFn: func(c Coordinate) []Coordinate {
			n := c.Neighbours()
			return slices.DeleteFunc(n, func(c Coordinate) bool {
				return !isWalkable(c.x, c.y, ActualSeed)
			})
		},

		// Note: Unused for BFS.
		CostToFinishFn: func(c Coordinate) int { return -1 },

		IsFinishFn: func(c Coordinate) bool {
			return c == finish
		},
	})

	f.MaxCost = 50
	f.Walk()

	for _, e := range f.EventLog() {
		if v, ok := e.(pathfind.EventCandidateVisited[Coordinate]); ok {
			visited[v.CandidateID] = struct{}{}
		}
	}

	// pull visited nodes from event log
	fmt.Printf("answer part two = %d\n", len(visited))
}

func isWalkable(x, y, seed int) bool {
	cacheKey := fmt.Sprintf("%d-%d", x, y)
	if walkable, found := floor[cacheKey]; found {
		return walkable
	}

	key := x*x + 3*x + 2*x*y + y + y*y + seed
	walkable := bits.OnesCount(uint(key))%2 == 0
	floor[cacheKey] = walkable
	return walkable
}

func main() {
	partOne()
	partTwo()
}
