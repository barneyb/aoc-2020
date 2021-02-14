package com.barneyb.aoc.aoc2020;

import lombok.AllArgsConstructor;
import lombok.Data;

import java.util.Map;

public class Aoc2020Day23CrabCups {

    private static final String EXAMPLE_ONE = "389125467";
    private static final String MY_INPUT = "469217538";

    @Data
    @AllArgsConstructor
    private static class Link {
        final long value;
        Link next;

        @Override
        public String toString() {
            var sb = new StringBuilder();
            sb.append("Link(").append(value + 1).append(')');
            var curr = this;
            for (int i = 0; i < 20 && curr.next != null && curr.next != this; i++) {
                curr = curr.next;
                sb.append(">").append(curr.value + 1);
            }
            return sb.toString();
        }
    }

    private static Link tick(final Link curr, int size) {
        // pick up three cups
        var picked_up = curr.next;
        curr.next = picked_up.next.next.next;
        picked_up.next.next.next = null;

        // find the destination label
        var dest_num = curr.value;
        do {
            dest_num = (dest_num + size - 1) % size;
        } while (picked_up.value == dest_num || picked_up.next.value == dest_num || picked_up.next.next.value == dest_num);

        // find the destination cup
        var dest_link = curr.next;
        while (dest_link.value != dest_num) {
            dest_link = dest_link.next;
        }

        // insert the picked up cups
        picked_up.next.next.next = dest_link.next;
        dest_link.next = picked_up;
        return curr.next;
    }

    private static long partOne(String input) {
        var curr = buildRing(input);
        for (int i = 0; i < 100; i++) {
            curr = tick(curr, input.length());
        }
        while (curr.value != 0) {
            curr = curr.next;
        }
        int result = 0;
        for (int i = input.length() - 1; i > 0; i--) {
            curr = curr.next;
            result *= 10;
            result += curr.value + 1;
        }
        return result;
    }

    private static long partTwo(String input, int cupCount, int moveCount) {
        var curr = buildRing(input, cupCount);
        var one = curr;
        // it's fast to find it now, since we know it's in the first few
        while (one.value != 0) {
            one = one.next;
        }
        for (int i = 0; i < moveCount; i++) {
            curr = tick(curr, cupCount);
        }
        return one.next.value * one.next.next.value;
    }

    private static <T extends Comparable<T>> String drawHistogram(Map<T, Integer> hist) {
        var max = hist.values().stream().reduce(Integer.MIN_VALUE, Math::max);
        var min = hist.values().stream().reduce(Integer.MAX_VALUE, Math::min);
        var sb = new StringBuilder();
        hist.keySet().stream().sorted().forEach(k -> {
            var label = k.toString();
            if (label.length() > 10) {
                label = label.substring(0, 10);
            }
            if (label.length() < 10) {
                sb.append(" ".repeat(10 - label.length()));
            }
            var val = hist.get(k);
            var bar = (val - min) * 60 / (max - min);
            sb.append(label).append(" |")
                    .append("#".repeat(bar))
                    .append(" ".repeat(60 - bar))
                    .append("| ")
                    .append(val)
                    .append('\n');
        });
        sb.append(hist.size())
                .append(" keys, ranging from ")
                .append(min)
                .append(" to ")
                .append(max)
                .append(" occurrences.");
        return sb.toString();
    }

    private static Link buildRing(String input) {
        return buildRing(input, 0);
    }

    private static Link buildRing(String input, int cupCount) {
        Link head = null;
        Link curr = null;
        for (int i = 0, len = input.length(); i < len; i++) {
            var l = new Link(input.charAt(i) - '0' - 1, null);
            if (head == null) {
                head = l;
                curr = l;
                head.next = curr;
            } else {
                curr.next = l;
                l.next = head;
                curr = l;
            }
        }
        // these aren't off-by-one
        for (int i = input.length(); i < cupCount; i++) {
            assert curr != null;
            curr.next = new Link(i, null);
            curr.next.next = head;
            curr = curr.next;
        }
        assert curr != null;
        return head;
    }

    public static void main(String[] args) {
        var start = System.currentTimeMillis();
        System.out.print("Part One: " + partOne(EXAMPLE_ONE));
        System.out.println(" (" + (System.currentTimeMillis() - start) + " ms)");
        var tickCount = 100_000; // 10_000_000
        var cupCount = 100_000; // 1_000_000
        start = System.currentTimeMillis();
        System.out.print("Part Two: " + partTwo(EXAMPLE_ONE, cupCount, tickCount));
        var elapsed = System.currentTimeMillis() - start;
        System.out.println(" (" + elapsed + " ms)");
        System.out.println("expect " + (elapsed * 1_000_000 / 1_000 * 10_000_000 / cupCount / tickCount) + "s to solve part two");
    }

}
