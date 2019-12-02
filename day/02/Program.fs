let usage = """Day 2: 1202 Program Alarm
Usage: 01 <input-file>
       01 --help
Options:
    -h, --help  Print this help.
"""

open System
open System.IO

open DocoptNet
open Xunit
open FsUnit.Xunit

type OpCode =
    | ADD = 1
    | MUL = 2
    | HALT = 99

let (|Instruction|) =
    function
    | OpCode.ADD -> (Some (+), 4, true)
    | OpCode.MUL -> (Some (*), 4, true)
    | OpCode.HALT -> (None, 1, false)
    | c -> failwithf "%A is invalid" c

let runProgram code =
    let code' = Array.copy code

    let rec loop ip =
        let (Instruction(operation, length, run)) = enum<OpCode> (code'.[ip])
        match operation with
        | Some op ->
            let op1 = code'.[ip + 1]
            let op2 = code'.[ip + 2]
            let storeAt = code'.[ip + length - 1]
            code'.[storeAt] <- op code'.[op1] code'.[op2]
        | None -> ()
        if run then loop (ip + length) else ()
    loop 0

    code'

module Tests =
    [<Fact>]
    let Intcode() =
        (*
            1,9,10,3,2,3,11,0,99,30,40,50
            1,0,0,0,99 becomes 2,0,0,0,99 (1 + 1 = 2).
            2,3,0,3,99 becomes 2,3,0,6,99 (3 * 2 = 6).
            2,4,4,5,99,0 becomes 2,4,4,5,99,9801 (99 * 99 = 9801).
            1,1,1,4,99,5,6,0,99 becomes 30,1,1,4,2,5,6,0,99.
        *)
        runProgram [| 1; 0; 0; 0; 99 |] |> should equal [| 2; 0; 0; 0; 99 |]
        runProgram [| 2; 3; 0; 3; 99 |] |> should equal [| 2; 3; 0; 6; 99 |]
        runProgram [| 2; 4; 4; 5; 99; 0 |] |> should equal [| 2; 4; 4; 5; 99; 9801 |]
        runProgram [| 1; 1; 1; 4; 99; 5; 6; 0; 99 |] |> should equal [| 30; 1; 1; 4; 2; 5; 6; 0; 99 |]
        runProgram [| 1; 9; 10; 3; 2; 3; 11; 0; 99; 30; 40; 50 |]
        |> should equal [| 3500; 9; 10; 70; 2; 3; 11; 0; 99; 30; 40; 50 |]

[<EntryPoint>]
let main argv =
    let args = Docopt().Apply(usage, argv, exit = true)
    let inputFile = args.["<input-file>"].ToString()
    let inputs = File.ReadAllText(inputFile).Split(',') |> Array.map (fun s -> Int32.Parse(s))

    inputs.[1] <- 12
    inputs.[2] <- 2
    let result = (runProgram inputs).[0]
    printfn "The result of the program is %i" result

    let result =
        (seq {
            for noun in 0 .. 99 do
                for verb in 0 .. 99 do
                    inputs.[1] <- noun
                    inputs.[2] <- verb
                    let res = (runProgram inputs).[0]

                    if res = 19690720 then yield 100 * noun + verb else ()
         }
         |> Seq.head)

    printfn "The input is %i" result
    0
