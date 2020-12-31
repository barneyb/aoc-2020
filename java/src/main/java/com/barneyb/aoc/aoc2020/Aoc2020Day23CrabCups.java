package com.barneyb.aoc.aoc2020;

import lombok.AllArgsConstructor;
import lombok.Data;

public class Aoc2020Day23CrabCups {

    private static final String EXAMPLE_ONE = "389125467";
    private static final String MY_INPUT = "469217538";

    @Data
    @AllArgsConstructor
    private static class Link {
        int value;
        Link next;

        @Override
        public String toString() {
            var sb = new StringBuilder();
            sb.append("Link(").append(value + 1).append(')');
            var curr = this;
            for (int i = 0; i < 10 && curr.next != null && curr.next != this; i++) {
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
        int dest_num = curr.value;
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

    private static long doIt(String input) {
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

    private static Link buildRing(String input) {
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
        assert curr != null;
        return head;
    }

    public static void main(String[] args) {
        System.out.println(doIt(EXAMPLE_ONE));
        System.out.println(doIt(MY_INPUT));
    }

}
