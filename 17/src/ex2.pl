% Opcodes
opcode(adv, 0).  % A = A >> shift
opcode(bxl, 1).  % B = B ^ literal
opcode(bst, 2).  % B = value % 8
opcode(jnz, 3).  % if A≠0 jump to operand
opcode(bxc, 4).  % B = B ^ C
opcode(out, 5).  % output value % 8
opcode(bdv, 6).  % B = A >> shift
opcode(cdv, 7).  % C = A >> shift

% ComboOperand values
combo_value(0, _, _, _, 0).     % Zero
combo_value(1, _, _, _, 1).     % One
combo_value(2, _, _, _, 2).     % Two
combo_value(3, _, _, _, 3).     % Three
combo_value(4, A, _, _, A).     % Register A
combo_value(5, _, B, _, B).     % Register B
combo_value(6, _, _, C, C).     % Register C
combo_value(7, _, _, _, _) :- fail.  % Reserved

execute([A, B, C], Op, Arg, [NewA, NewB, NewC], Out) :-
    opcode(Type, Op),
    execute_op(Type, Arg, [A, B, C], [NewA, NewB, NewC], Out).

execute_op(adv, Shift, [A, B, C], [NewA, B, C], none) :-
    combo_value(Shift, A, B, C, ShiftVal),
    NewA is A >> ShiftVal.

execute_op(bxl, Lit, [A, B, C], [A, NewB, C], none) :-
    NewB is B xor Lit.

execute_op(bst, Val, [A, B, C], [A, NewB, C], none) :-
    combo_value(Val, A, B, C, ComboVal),
    NewB is ComboVal mod 8.

execute_op(jnz, Target, [A, B, C], [A, B, C], jump(Target)) :-
    A \= 0.

execute_op(jnz, _, [0, B, C], [0, B, C], next).

execute_op(bxc, _, [A, B, C], [A, NewB, C], none) :-
    NewB is B xor C.

execute_op(out, Val, [A, B, C], [A, B, C], out(OutVal)) :-
    combo_value(Val, A, B, C, ComboVal),
    OutVal is ComboVal mod 8.

execute_op(bdv, Shift, [A, B, C], [A, NewB, C], none) :-
    combo_value(Shift, A, B, C, ShiftVal),
    NewB is A >> ShiftVal.

execute_op(cdv, Shift, [A, B, C], [A, B, NewC], none) :-
    combo_value(Shift, A, B, C, ShiftVal),
    NewC is A >> ShiftVal.

run_program(Program, State, Output) :-
    run_program(Program, State, 0, [], Output).

run_program(Program, State, IP, Acc, Output) :-
    length(Program, Len),
    (IP >= Len ->
        reverse(Acc, Output), !
    ;
        nth0(IP, Program, Op),
        NextIP is IP + 1,
        (NextIP >= Len ->
            reverse(Acc, Output), !
        ;
            nth0(NextIP, Program, Arg),
            execute(State, Op, Arg, NewState, Action),
            handle_action(Action, IP, NewIP),
            (Action = out(Val) ->
                run_program(Program, NewState, NewIP, [Val|Acc], Output)
            ;
                run_program(Program, NewState, NewIP, Acc, Output)
            )
        )
    ).

handle_action(none, IP, NewIP) :- NewIP is IP + 2.
handle_action(next, IP, NewIP) :- NewIP is IP + 2.
handle_action(jump(Target), _, Target).
handle_action(out(_), IP, NewIP) :- NewIP is IP + 2.

find_minimal_a_for_program(Program, [B, C], A) :-
    numlist(1, 1000000, Candidates),
    member(A, Candidates),
    run_program(Program, [A, B, C], Program),
    !.  % Cut after first solution since we want minimal

:- begin_tests(program).

test(out_1) :-
    run_program([5, 0], [0, 0, 0], [0]).

test(out_2) :-
    run_program([5, 1], [0, 0, 0], [1]).

test(out_3) :-
    run_program([5, 2], [0, 0, 0], [2]).

test(out_4) :-
    run_program([5, 3], [0, 0, 0], [3]).

test(out_5) :-
    run_program([5, 4], [42, 0, 0], [2]).

test(out_6) :-
    run_program([5, 5], [0, 13, 0], [5]).

test(out_7) :-
    run_program([5, 6], [0, 0, 27], [3]).

test(ex_1_example) :-
    run_program([0,1,5,4,3,0], [729,0,0], Output),
    Output = [4,6,3,5,6,3,5,2,1,0], !.

test(minimal_a_example) :-
    find_minimal_a_for_program([0,3,5,4,3,0], [0, 0], 117440).

:- end_tests(program).


% Register A: 52884621
% Register B: 0
% Register C: 0

% Program: 2,4,1,3,7,5,4,7,0,3,1,5,5,5,3,0

solve_ex2 :-
    find_minimal_a_for_program([2,4,1,3,7,5,4,7,0,3,1,5,5,5,3,0], [0, 0], A),
    writeln(A).
