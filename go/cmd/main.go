package main

import ("fmt" 
        "bufio"
        "os"
        "unicode"
        "strconv"
        "slices"
    )
type Point struct {
    x int
    y int
    length int
    number int
}

type Location struct {
    x int
    y int
}

func main() {

    filePath := "./input/3p1.txt"

    file, err := os.Open(filePath)
    if err != nil {
        fmt.Println(err)
        return
    }
    defer file.Close()

    scanner := bufio.NewScanner(file)
    ln := 0
    loc := make(map[int]Point)
    pts_ := make(map[int]Point)
    for scanner.Scan() {
        
        ln++
        pts, spPts := linefeed(scanner.Text(), ln)
        for i := range pts {
            pts_[len(pts_)] = pts[i]
        }
        for i := range spPts {
            loc[len(loc)] = spPts[i]
        }
    }
    
    total := 0
    nums := make(map[int]Location)
    for i := range loc {
        
        valx := []int{}
        valy := []int{}

        valx = append(valx, loc[i].x)
        valx = append(valx, loc[i].x+1)
        valx = append(valx, loc[i].x+2)

        valy = append(valy, loc[i].y-1)
        valy = append(valy, loc[i].y)
        valy = append(valy, loc[i].y+1)
        
        for j := range pts_ {
            length := pts_[j].length

            interx := []int{}
            interx = append(interx, pts_[j].x)
            interx = append(interx, pts_[j].x-1)
            interx = append(interx, pts_[j].x+1)
            if length >= 3{
                interx = append(interx, pts_[j].x-2)
            }
            if slices.Contains(valy, pts_[j].y){
                for x := range interx {
                    if slices.Contains(valx, interx[x]){
                        if r, ok := nums[pts_[j].number]; ok {
                            if r.x == pts_[j].x && r.y == pts_[j].y {
                                break 
                            } else {
                                total += pts_[j].number
                                nums[pts_[j].number] = Location{pts_[j].x, pts_[j].y}
                                break
                            }
                        } else {
                            nums[pts_[j].number] = Location{pts_[j].x, pts_[j].y}
                            total += pts_[j].number
                            break
                        }
                    }
                }
            }
        }
    }
    fmt.Println("Total: ", total)

    if err := scanner.Err(); err != nil {
        fmt.Println(err)
    }
    
}

func linefeed(line string, lineNum int) (map[int]Point, map[int]Point){
    tmp := ""
    point := Point{-1, lineNum, 0, 0}
    points := make(map[int]Point)
    specialPoints := make(map[int]Point)
    //special := "!@#$%&^*()+/-=<>?^_~"
    counter := 0
    for char := range line {
        point.x++ 
        char := line[char]
        counter++

        if unicode.IsDigit(rune(char)) {
            if counter == len(line) {
                point.length++
                tmp += string(rune(char))
                point.number, _ = strconv.Atoi(tmp)
                points[len(points)] = point
                tmp = ""
                continue
            } 
            tmp += string(rune(char))
            point.length++
            continue
        }
        if !unicode.IsDigit(rune(char)) && len(tmp) > 0{
            if string(char) != "."{
                point.number, _ = strconv.Atoi(tmp)
                points[len(points)] = point
                point.length = 0
                specialPoints[len(specialPoints)] = point
                tmp = ""
                continue
            } else {
                point.number, _ = strconv.Atoi(tmp)
                points[len(points)] = point
                point.length = 0
                tmp = ""
                continue
            }
        } else {
            if string(char) != "."{
                point.length = 0
                specialPoints[len(specialPoints)] = point
                tmp = ""
                continue
            }
            tmp = ""
            continue
        }
    }

    for i := range points {
        fmt.Println("Point: ", points[i])
    }

    return points, specialPoints
}
