# Day11

This took waaay to long to solve 😳

```bash
iex -S mix

iex(1)> Day11.solve

# 🕘

{:found,
 %State{
   cost: 31,
   elevator_location: 3,
   floors: [               
     %Floor{slots: #MapSet<[]>},            
     %Floor{slots: #MapSet<[]>},
     %Floor{slots: #MapSet<[]>},
     %Floor{
       slots: #MapSet<[  
         %Generator{identifier: :plutonium},
         %Generator{identifier: :promethium},
         %Generator{identifier: :ruthenium},
         %Generator{identifier: :strontium},
         %Generator{identifier: :thulium},
         %Microchip{identifier: :plutonium},
         %Microchip{identifier: :promethium},
         %Microchip{identifier: :ruthenium},
         %Microchip{identifier: :strontium},
         %Microchip{identifier: :thulium}
       ]>
     }
   ],
   hash: "8DB9CBDC52AACFBBDB0F61319858738F54EEAE3E8D28B246419E82258EA5A306",
   hops: 31
 }
}
```

## Issues

- Had a bug in my hashing function so collisions would happen thus many, many states would be skipped and it would terminate without finding the answer.

- BFS works but A* is probably quicker, I will attempt that for Part B.
