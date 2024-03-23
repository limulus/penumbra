---
tags: journal
layout: penumbra-journal-entry
title: 'How I Am Using GitHub Copilot'
date: 2024-03-23 15:15:00 -07:00
image: poster.jpeg
teaser: >-
  A six minute video I produced to show how I am using GitHub Copilot as I work on
  my ray tracer, Penumbra. Includes a preview of the upcoming switch to Rust/WebAssembly!
---

<video-on-demand vod="github-copilot-demo/01HSMMN4H9QZ0QGYPFCWSYQVWF"></video-on-demand>

<script type="module" src="../../assets/js/video-on-demand/index.js"></script>

Above is a video I produced to demo GitHub Copilot to my coworkers. If you haven’t yet
explored using a Large Language Model to you help you code, it is worth a watch. I
screen-recorded myself developing an optimization for this project and edited it down to
about 6 minutes.

I use Copilot in other ways not included in the video. It’s clearly been trained on other
Ray Tracer Challenge implementations so it very quickly autocompletes tests with all the
exact values. This has saved me a bunch of mindless typing. It also autocompletes production
code that satisfies the tests, which is often less helpful for this project since I usually
want to spend some time thinking about how to implement these things.

If you watch the video you’ll notice that I’ve switched to Rust targeting WebAssembly.
There’s a story behind that! But it will have to wait for a future post.

Making this video was a lot of fun! So much so that I am routinely recording myself working
on things and spent some time setting up some video hosting infrastructure and a rudimentary
pipeline.
