module Day01

function getrequiredfuel(mass::Integer)
    floor(Int, mass / 3) - 2
end


function getrequiredfuel2(mass::Integer)
    total = 0
    while true
        mass = getrequiredfuel(mass)
        if mass <= 0
            break
        end
        total += mass
    end
    total
end

end
