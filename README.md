# Contraction Hierarchies

This repository contains a Rust implementation of [Contraction Hierarchies](https://en.wikipedia.org/wiki/Contraction_hierarchies) a speed-up technique for finding the shortest path in a graph.
The project is part of the [Algorithm Engineering Lab - Route Planning](https://i11www.iti.kit.edu/teaching/winter2023/algorithmengineeringpraktikum/index) at ITI Karlruher Institute of Technology.
In a preprocessing step, shortcuts that skip over "unimportant" nodes are computed based on a node ordering.
The shortest path can be computed using a bidirectional Dijkstra search in the augmented graph that only follows edges upward in the node hiearchy.
Contraction Hierarchies leverage the fact that road networks tend to be highly hierarchical.


**Example**
![ch](/images/ch.png)

# References

[German lecture slides on CH from "Algorithms for Route Planning" 2022 course at ITI KIT.](https://i11www.iti.kit.edu/_media/teaching/sommer2022/routenplanung/chap1-ch.pdf)

[Robert Geisberger, Peter Sanders, Dominik Schultes, and Christian Vetter.
Exact routing in large road networks using contraction hierarchies.
Transportation Science, 46(3):388–404, August 2012.](https://publikationen.bibliothek.kit.edu/1000028701)