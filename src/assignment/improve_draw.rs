// Hide the cursor before you refresh the screen by using Hide.
// 隐藏刷新屏幕前的光标，请使用 Hide。
// Show the cursor after you’re done refreshing by using Show.
// 显示刷新完成后光标，使用“显示”。
// Clear only what you intend to redraw by using an appropriate ClearType.
// 仅清除您打算使用适当 ClearType 重新绘制的部分
// Refactor the current implementation to no longer directly use print!, but use crossterm’s Print instead.
// 重构当前实现，不再直接使用 print! ，而是使用 crossterm 的 Print。
// Refactor the current implementation to no longer use execute!. Use queue! instead, which you can use exactly the same. You’ll need to manually ensure a write by calling stdout.flush(); to trigger the write at the appropriate time. Hint: To call this, you need to use std::io::Write;.
// 重构当前实现，不再使用 execute! 。使用 queue! 代替，你可以完全相同地使用它。你需要手动确保写入，通过调用 stdout.flush(); 在适当的时间触发写入。提示：要调用此功能，你需要 use std::io::Write; 。
// Bonus Refactor: size() returns a pair of two ordered values, called a tuple. Wouldn’t it be nicer if we somehow worked with a struct, so that we could make it make it more explicit which of the two values is the height and which one is the width?
// 奖金重构： size() 返回一对有序值，称为元组。如果我们可以以某种方式使用 struct ，以便更明确地表示两个值中的哪一个是 height ，哪一个又是 width ，那岂不是更好？
// Bonus Refactor 2: MoveTo works with two parameters to denote the x and y position. Wouldn’t it be nicer if we somehow worked with a struct, so that we can make it more explicit which of these two values is the x and which is the y position?
// 奖金重构 2： MoveTo 使用两个参数来表示 x 和 y 位置。如果我们可以用某种方式使用 struct ，以便更明确地表示这两个值中的哪个是 x，哪个是 y 位置，那不是更好吗？