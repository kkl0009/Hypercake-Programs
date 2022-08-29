(* Name: Kollin Labowski
  Course: CS 310
  Description: This program implements the HyperCake problem using the Pascal programming language.
  Academic Integrity: This program complies with the academic integrity policies of WVU and CS 310

  hypercake function uses recursion and two embedded methods combinations and factorial
*)

program HypercakeProblem;

function hypercake(n, k: cardinal): cardinal;
    function combinations(n, r: cardinal): cardinal;
        function factorial(n: cardinal): cardinal;
        begin
            if(n <= 1) then
                factorial := 1
            else
                factorial := n * factorial(n - 1);
        end;
    
    begin
        combinations := factorial(n) div (factorial(r) * factorial(n - r));
    end;
    
begin
    if(k <= 0) then
        hypercake := combinations(n, k)
    else
        hypercake:= combinations(n, k) + hypercake(n, k - 1);
end;


var n : cardinal;
    k : cardinal;
begin
  writeln ('Enter the amount of cuts (n): ');
  readln(n);
  writeln ('Enter the amount of dimensions (k): ');
  readln(k);
  writeln('The answer is: ', hypercake(n, k));
end.