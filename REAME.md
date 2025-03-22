# Simple Protection System

## BRIEF

本仓库实现了一个基于Take-Grant模型与Object-Capability模型的简单权能系统。

## STRUCTURE

结构简介如下

- right.rs
  - 权限，能力所支持的操作。
  - 支持：读、写、执行
- object.rs
  - 对象，权能的主体
  - 实现留空，仅作为唯一个体的指代。
- capability.rs
  - 能力，描述了特定对象对特定对象执行操作的可转移权利。
  - 存储能力的主体、客体和权限
- graph.rs
  - 权能关系图，采用邻接表法构造与存储。
  - 提供take-grant model的操作与算法接口。

## TEST

tests目录下为对应的模块集成测试。

## REFERENCE

- R. J. Lipton and L. Snyder. 1977. A Linear Time Algorithm for Deciding Subject Security. J. ACM 24, 3 (July 1977), 455–464. https://doi.org/10.1145/322017.322025