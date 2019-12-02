let usage = """Day 1: The Tyranny of the Rocket Equation
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


let getRequiredFuel mass =
    mass / 3 - 2

let rec getRequiredFuelExp mass =
    let fuel = getRequiredFuel mass
    if fuel > 0 then
        fuel + getRequiredFuelExp fuel
    else
        0

module Tests =
    [<Fact>]
    let ``test required fuel`` () =
        (* For a mass of 12, divide by 3 and round down to get 4, then subtract 2 to get 2.
           For a mass of 14, dividing by 3 and rounding down still yields 4, so the fuel required is also 2.
           For a mass of 1969, the fuel required is 654.
           For a mass of 100756, the fuel required is 33583.
        *)
        getRequiredFuel 12 |> should equal 2
        getRequiredFuel 14 |> should equal 2
        getRequiredFuel 1969 |> should equal 654
        getRequiredFuel 100756 |> should equal 33583
        
    [<Fact>]
    let ``test required fuel exponential`` () =
        (* A module of mass 14 requires 2 fuel. This fuel requires no further fuel (2 divided by 3 and rounded down is 0,
           which would call for a negative fuel), so the total fuel required is still just 2.
           At first, a module of mass 1969 requires 654 fuel. Then, this fuel requires 216 more fuel (654 / 3 - 2).
           216 then requires 70 more fuel, which requires 21 fuel, which requires 5 fuel, which requires no further fuel.
           So, the total fuel required for a module of mass 1969 is 654 + 216 + 70 + 21 + 5 = 966.
           The fuel required by a module of mass 100756 and its fuel is: 33583 + 11192 + 3728 + 1240 + 411 + 135 + 43 + 12 + 2 = 50346.
        *)
        getRequiredFuelExp 14 |> should equal 2
        getRequiredFuelExp 1969 |> should equal 966
        getRequiredFuelExp 100756 |> should equal 50346

[<EntryPoint>]
let main argv =
    let args = Docopt().Apply(usage, argv, exit = true)
    let inputFile = args.["<input-file>"].ToString()
    let inputs = File.ReadAllLines(inputFile) |> Array.map (fun l -> Int32.Parse(l))

    let result = inputs |> Array.map getRequiredFuel |> Array.sum
    printfn "The required fuel for all the parts is %i" result

    let result = inputs |> Array.map getRequiredFuelExp |> Array.sum
    printfn "The required fuel for all the parts and fuel is %i" result
    0