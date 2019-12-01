#!/usr/bin/env julia

include("lib.jl")

function sumfuel(f)
    lines = open("$(@__DIR__)/../input") do f
        lines = readlines(f)
        map(l -> parse(Int, l), lines)
    end
    sum(map(m -> f(m), lines))
end

println("The answer to the first part is $(sumfuel(Day01.getrequiredfuel))")
println("The answer to the second part is $(sumfuel(Day01.getrequiredfuel2))")