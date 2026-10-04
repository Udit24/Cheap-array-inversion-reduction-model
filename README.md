# Cheap-array-inversion-reduction-model
A very cheap statistical model to reduce an arbitrary arrays inversion count enough that a second algorithm becomes linear
Assuming no outliers exist and the data is mostly within a range.
The goal is to create two teams, a team of larger of the values in the set and another otherwise.
Both teams pull an element in both directions to place it in a position where it might be after getting sorted.
But the job is lousy and it doesnt do it well
We check it for errors and it repeats with a feedback to do it better the next time.
The Goal is to get near linear time and stop at a level of satisfaction for the user or hand over to a sorting algorithm that performs really well with near sorted data ( Like inserion sort )
