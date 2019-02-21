# rust-algorithms

> 本项目还在施工中...

所有排序使用的数据为(0, 10000)的`i32`类型数字.如果你对于下面某些排序有疑问,可以去[www.cs.usfca.edu](https://www.cs.usfca.edu/~galles/visualization/ComparisonSort.html)以动画形式查看各种排序,非常简单易懂.

## 冒泡排序

实现了三种冒泡方式,每次都以上一次进行优化:

- 默认冒泡
- 优化1 (若一次冒泡没有元素进行交互位置,那么说明已经排序好了)
- 优化1+2 (在1优化的前提下记录上次交互元素的位置,下一次循环到这个元素位置即可)

```
bubble  take time PT6.919733701S
bubble1 take time PT6.687554709S
bubble2 take time PT6.535498605S
```

可见优化后差距并不大,总体都在6.5秒之间.但是数据量提升一个数量级后时间会增加很多,比如(0,100000)的数据量使用冒泡是非常久的.

## 选择排序

```
select take time PT4.073199300S
```