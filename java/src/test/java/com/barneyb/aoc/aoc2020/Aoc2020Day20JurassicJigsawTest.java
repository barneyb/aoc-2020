package com.barneyb.aoc.aoc2020;

import com.barneyb.aoc.aoc2020.Aoc2020Day20JurassicJigsaw.Tile;
import org.junit.jupiter.api.Test;

import java.util.Arrays;

import static com.barneyb.aoc.aoc2020.Aoc2020Day20JurassicJigsaw.Dir.*;
import static com.barneyb.aoc.aoc2020.Aoc2020Day20JurassicJigsaw.partOne;
import static org.junit.jupiter.api.Assertions.assertEquals;

class Aoc2020Day20JurassicJigsawTest {

    private Tile[] exampleTiles() {
        return new Tile[] {
            new Tile(2311, "..##.#..#.##..#.....#...##..#.####.#...###.##.###.##...#.###.#.#.#..##..#....#..###...#.#...###..###"),
            new Tile(1951, "#.##...##.#.####...#.....#..###...######.##.#....#.###.########.##.##..###....#...#.#..#.##...##.#.."),
            new Tile(1171, "####...##.#..##.#..###.#..#.#..###.####...###.####.##....##..#...####.#.##.####.####..#........##..."),
            new Tile(1427, "###.##.#...#..#.##...#.##.#..##.#.#.##.#....#...##...##..##....#.#####.#.####.#...#..###.#..##.#..#."),
            new Tile(1489, "##.#.#......##...#...##..##.....#...#...#####...#.#..#.#.#.#...#.#.#..##.#...##...##.##.#####.##.#.."),
            new Tile(2473, "#....####.#..#.##...#.##..#...######.#.#.#...#.#.#.#########.###.#..#.########.###...##.#...###.#.#."),
            new Tile(2971, "..#.#....##...###...#.#.###...##.##..#...#####..##.#..####.##..#.#..#...####.###..#.#.###....#.#.#.#"),
            new Tile(2729, "...#.#.#.#####.#......#.#.........#..#.#.##..##.#..#.####...####.#.#..##.####...##..#.##..#.##...##."),
            new Tile(3079, "#.#.#####..#..######..#.......######....####.#..#..#...#.##.#.#####.##..#.###.....#.........#.###...")
       };
    }

    private Tile getTile2311() {
        return exampleTiles()[0];
    }

    private Tile getTile3079() {
        //noinspection OptionalGetWithoutIsPresent
        return Arrays.stream(exampleTiles()).filter(t -> t.num == 3079).findFirst().get();
    }

    @Test
    public void testEdges() {
        var t = getTile2311();
        assertEquals("..##.#..#." +
                "##..#....." +
                "#...##..#." +
                "####.#...#" +
                "##.##.###." +
                "##...#.###" +
                ".#.#.#..##" +
                "..#....#.." +
                "###...#.#." +
                "..###..###", t.getPixels());
        assertEquals("..##.#..#.", t.getEdge(North));
        assertEquals("###..###..", t.getEdge(South));
        assertEquals("...#.##..#", t.getEdge(East));
        assertEquals(".#..#####.", t.getEdge(West));

        t = getTile3079();
        assertEquals("#.#.#####.", t.getEdge(North));
        assertEquals("...###.#..", t.getEdge(South));
        assertEquals(".#....#...", t.getEdge(East));
        assertEquals("...#.##..#", t.getEdge(West));
    }

    @Test
    public void testFlip() {
        var t = getTile2311();
        t.flip();
        assertEquals(".#..#.##.." +
                ".....#..##" +
                ".#..##...#" +
                "#...#.####" +
                ".###.##.##" +
                "###.#...##" +
                "##..#.#.#." +
                "..#....#.." +
                ".#.#...###" +
                "###..###..", t.getPixels());
        assertEquals(".#..#.##..", t.getEdge(North));
        assertEquals("..###..###", t.getEdge(South));
        assertEquals(".#####..#.", t.getEdge(East));
        assertEquals("#..##.#...", t.getEdge(West));

        t = getTile3079();
        t.flip();
        assertEquals(".#####.#.#", t.getEdge(North));
        assertEquals("..#.###...", t.getEdge(South));
        assertEquals("#..##.#...", t.getEdge(East));
        assertEquals("...#....#.", t.getEdge(West));
    }

    @Test
    public void testRotate() {
        var t = getTile2311();
        t.rotate();
        assertEquals(".#..#####." +
                ".#.####.#." +
                "###...#..#" +
                "#..#.##..#" +
                "#....#.##." +
                "...##.##.#" +
                ".#...#...." +
                "#.#.##...." +
                "##.###.#.#" +
                "#..##.#...", t.getPixels());
        assertEquals("..##.#..#.", t.getEdge(East));
        assertEquals("###..###..", t.getEdge(West));
        assertEquals("...#.##..#", t.getEdge(South));
        assertEquals(".#..#####.", t.getEdge(North));

        t = getTile3079();
        t.rotate();
        assertEquals("#.#.#####.", t.getEdge(East));
        assertEquals("...###.#..", t.getEdge(West));
        assertEquals(".#....#...", t.getEdge(South));
        assertEquals("...#.##..#", t.getEdge(North));
    }

    @Test
    public void testPartOne() {
        assertEquals(20899048083289L, partOne(exampleTiles()));
    }

}
