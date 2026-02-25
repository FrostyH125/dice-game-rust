//progress = clamp(t / duration, 0, 1)
//amplitude = 1.0 - progress

//offset = 0.0
//for i, char in enumerate(str):
    //x = pos.x + charWidth * i
    //y = pos.y + sin(t + offset) * amplitude * waveHeight
    //draw(char, x, y)
    //offset += 0.1
    
//smoother:
// amplitude = (1.0 - progress)^2
// or
// amplitude = pow(1.0 - progress, 3)