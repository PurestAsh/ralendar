# ralander
a cn calander build in rust and iced


widget只能表示当前帧的状态，而widget的state者可以表示
整个widget的生命周期都存在。
每一帧都会创建一个新的widget. 但是state只有在一开始创建。
