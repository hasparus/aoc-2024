read_input(File, Numbers) :-
    open(File, read, Stream),
    read_line_to_string(Stream, Line),
    close(Stream),
    split_string(Line, " ", "", StringNumbers),
    maplist(number_string, Numbers, StringNumbers).



main :-
    read_input('input.txt', Stones),
    solve(Stones, 25, FirstSolution),
    writeln(FirstSolution),
    solve(Stones, 75, SecondSolution),
    writeln(SecondSolution).



even_number_of_digits(Number) :-
    Number > 0,
    Digits is floor(log10(Number)) + 1,
    0 is Digits mod 2.



transform(0, [1]).

transform(Number, Result) :-
    Number > 0,
    even_number_of_digits(Number),
    Digits is floor(log10(Number)) + 1,
    Mid is 10 ** (Digits // 2),
    Left is Number // Mid,
    Right is Number mod Mid,
    Result = [Left, Right].

transform(Number, Result) :-
    Number > 0,
    \+ even_number_of_digits(Number),
    Multiplied is Number * 2024,
    Result = [Multiplied].



:- table solve/3.

solve([], _, 0).

solve(Stones, 0, Length) :-
    length(Stones, Length).

solve([Head|Tail], Iterations, Length) :-
    Iterations > 0,
    NextIter is Iterations - 1,
    transform(Head, TransformedList),
    solve(TransformedList, NextIter, HeadLength),
    solve(Tail, Iterations, TailLength),
    Length is HeadLength + TailLength.



:- begin_tests(stones).

test(transform_zero) :-
    transform(0, [1]).

test(transform_odd) :-
    transform(1, [2024]).

test(transform_even) :-
    transform(1234, [12, 34]).

test(solve_trivial) :-
    solve([125, 17], 1, 3).

test(solve_example) :-
  solve([125, 17], 25, 55312).

:- end_tests(stones).


