My name is Yann Youbi, I work individually.

A TA helped me clear out some misunderstanding regarding what was expected from me for the part A of the assignment and the design.

I think I could implement my sudoku better by iterating in column major and row major at the same time with paralelel iteration.
I could maybe zip the iterations so that I traverse my 2 dimensional array in diagonal while checking each row and column at each diagonal iteration.

I can't really say much about what has been correctly implemented because I believe everything could be better implemented one way or another and I've just started using rust a month ago

design checklist: 
I use a structure for the 2 dimensional array with a width and a height as a tuple of usize elements as  well as a vector of elements of typle T and a tracker tuple of 2 usize elements for the column and row indexes
We implement an iterator with a tuple item of 2 usizes and a next function
for each item it increments the column and/or row tracker following row major order
we implement 2 functions taking an 2D array using our next function and returning an iterator over of vector of tuples with 3 usizes: column and row indexes followed by a corresponding 1 dimmensional index.
for every element of our 2D array it calls the next function and store it inside of a vector of tuple of usizes
we implement a constructor with a new function taking a vector of type T elements and a tuple of usizes for the width and the height of the 2D array

Type spent:
I have spent about a day working on the assignment design document included. 
About a night for the 2d array implementation (part A), about 6 hours on the design document and another night for the sudoku checker program(part B and part C)
I will definitively spend some more time to make sure everything is actually done and to try to improve my code as much as possinle.
