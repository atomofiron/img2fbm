# img2fbm
Image to Flipper bitmap converter

https://github.com/Atomofiron/img2fbm/assets/14147217/59cbb785-d17d-46e0-a8fe-b8a8210959ec

# Functionality

<details>
  <summary>img2fbm --help</summary>

```
Flipper bitmap files generator

Usage: img2fbm [OPTIONS] <source> [dolphin]

Arguments:
  <source>
          Path to png|jpg|jpeg|gif file

  [dolphin]
          Path to the 'dolphin' directory, if the gif passed

Options:
  -H, --height <1-64>
          Sets the height of output frame(s)
          
          [default: 64]

      --st <type>
          Scale type
          
          [default: fit]

          Possible values:
          - fill: Scale to fill animation bounds
          - fit:  Scale to fit in animation bounds

  -a, --alignment <side>
          Applied alignment if the source picture has aspect ratio different from the target
          
          [default: bottom]

          Possible values:
          - left:   Align source picture to left
          - top:    Align source picture to top
          - right:  Align source picture to right
          - bottom: Align source picture to bottom

  -p, --preview
          Generate the previews of result pictures

      --op
          Only preview, do not generate .bm and other Flipper Animation files

      --ps <multiplier>
          Preview scale ratio
          
          [default: 3]

  -i, --inverse
          Inverse output pixels

  -r, --replace-manifest
          Replace dolphin/manifest.txt file with a new one

  -b, --background <background>
          Set background pixels visible
          
          [default: invisible]

          Possible values:
          - invisible: Keep transparent, white, unset, zero
          - start:     Make visible on the left or top side
          - end:       Make visible on the right or bottom side
          - visible:   Make visible, black, set, unit

  -t, --threshold <percentage[:percentage]>
          Threshold value or range of pixel brightness as a percentage, such as 20:80, 40:, :60, 50:50 or 50
          
          [default: 20:80]

  -s, --speed <speed>
          Animation speed ratio
          
          [default: 1]

  -c, --cut <count[:count]>
          Drop some frames from the start and from the end. For example 5:, :8 or 2:3, the last one drops 2 frames from start and 3 from the end
          
          [default: 0:0]
```
</details>

# Download
From [Releases](https://github.com/Atomofiron/img2fbm/releases)
<br>:white_check_mark: MacOS x86_64
<br>:white_check_mark: MacOS ARM
<br>:white_check_mark: Linux x86_64
<br>:zzz: Linux ARM
<br>:white_check_mark: Windows x86_64

# Samples
<img src=https://github.com/Atomofiron/img2fbm/assets/14147217/efc04271-4cea-4a58-878a-38c83db58200 height=64 alt=yuno-eyes /> <img src=https://github.com/Atomofiron/img2fbm/assets/14147217/a2160d4e-5e24-414e-8a72-fc67b410df87 height=64 alt=yuno-eyes_preview />
<br>
<img src=https://github.com/Atomofiron/img2fbm/assets/14147217/a29d019b-a75c-407d-b957-3228ffdac3af height=64 alt=yuno-whisper />
<img src=https://github.com/Atomofiron/img2fbm/assets/14147217/5a5f8b1f-a6a6-4f21-8c67-09d4f9a73753 height=64 alt=yuno-whisper_preview />
<br>
<img src=https://github.com/Atomofiron/img2fbm/assets/14147217/006bea7f-bde2-4ca1-9236-7538c226da87 height=64 alt=yuno-shoot />
<img src=https://github.com/Atomofiron/img2fbm/assets/14147217/f9160543-3abd-4916-a8cc-ea93033f7589 height=64 alt=yuno-shoot_preview />
<br>
<img src=https://github.com/Atomofiron/img2fbm/assets/14147217/8b384c08-77b3-4e98-8d77-a5c77bce5e89 height=64 alt=yuno-shadow />
<img src=https://github.com/Atomofiron/img2fbm/assets/14147217/c00d93e3-27e3-43a1-bf87-e11bb1bbba36 height=64 alt=yuno-shadow_preview />
<br>
<img src=https://github.com/Atomofiron/img2fbm/assets/14147217/e25a9657-52d9-4440-8287-da271b10a8d2 height=64 alt=yuno-run />
<img src=https://github.com/Atomofiron/img2fbm/assets/14147217/fa408c48-f6c2-4b90-99b3-d0466ad54a56 height=64 alt=yuno-run_preview />
<br>
<img src=https://github.com/Atomofiron/img2fbm/assets/14147217/f0377d75-ab9c-466c-ac1a-5356fbee23eb height=64 alt=yuno-mysterious />
<img src=https://github.com/Atomofiron/img2fbm/assets/14147217/db4df409-013d-4e21-8638-73045bb65841 height=64 alt=yuno-mysterious_preview />
<br>
<img src=https://github.com/Atomofiron/img2fbm/assets/14147217/b1877adc-9a84-49da-ad55-6ffd3eb2f532 height=64 alt=yuno-knife />
<img src=https://github.com/Atomofiron/img2fbm/assets/14147217/f2b1d1dc-4fd8-47da-8a48-98557b4f0159 height=64 alt=yuno-knife_preview />
<br>
<img src=https://github.com/Atomofiron/img2fbm/assets/14147217/b2546d3e-0c0d-4254-bc99-82a747374d3a height=64 alt=yuno-katana />
<img src=https://github.com/Atomofiron/img2fbm/assets/14147217/708cb70a-7c62-4b86-a584-308234174a08 height=64 alt=yuno-katana_preview />
<br>
<img src=https://github.com/Atomofiron/img2fbm/assets/14147217/b945a37b-1cc6-4d8c-b38d-7aa34e4cdb6d height=64 alt=yuno-heh />
<img src=https://github.com/Atomofiron/img2fbm/assets/14147217/4ddae3cf-505a-4767-ad1b-ff51bb7617ca height=64 alt=yuno-heh_preview />
<br>
<img src=https://github.com/Atomofiron/img2fbm/assets/14147217/af9dd5fc-0344-453f-8b5a-db7973acced3 height=64 alt=yuno-final />
<img src=https://github.com/Atomofiron/img2fbm/assets/14147217/5e239de0-cb5b-4570-825c-1907c6ea7c9a height=64 alt=yuno-final_preview />
<br>
<img src=https://github.com/Atomofiron/img2fbm/assets/14147217/326055d9-20f3-494f-a246-fe333ba7aea0 height=64 alt=yuno-fight />
<img src=https://github.com/Atomofiron/img2fbm/assets/14147217/bda9d394-2043-43df-928b-c12a0d3540de height=64 alt=yuno-fight_preview />
<br>
<img src=https://github.com/Atomofiron/img2fbm/assets/14147217/ad108e71-7cc6-4395-a82a-6b20bcb085df height=64 alt=yuno-crying />
<img src=https://github.com/Atomofiron/img2fbm/assets/14147217/71f62107-5164-43c5-8620-ebe7f00b0a5e height=64 alt=yuno-crying_preview />
<br>
<img src=https://github.com/Atomofiron/img2fbm/assets/14147217/1065cd48-1374-4877-a646-1b608b5cc34e height=64 alt=yuno-confused />
<img src=https://github.com/Atomofiron/img2fbm/assets/14147217/fc92e7fd-ea35-4e2d-8080-415372ff732c height=64 alt=yuno-confused_preview />
<br>
<img src=https://github.com/Atomofiron/img2fbm/assets/14147217/a3bb221b-80ff-4816-a769-3394a1bd0368 height=64 alt=yuno-axe />
<img src=https://github.com/Atomofiron/img2fbm/assets/14147217/cb9fd2fe-2e16-493f-ab42-049e36279318 height=64 alt=yuno-axe_preview />
<br>
<img src=https://github.com/Atomofiron/img2fbm/assets/14147217/cbbbed4e-eacd-47bf-8c6d-3e7e7d57539c height=64 alt=yuno-afraid />
<img src=https://github.com/Atomofiron/img2fbm/assets/14147217/b6ccf9b0-0824-444d-9983-c952218a3778 height=64 alt=yuno-afraid_preview />
<br>
<img src=https://github.com/Atomofiron/img2fbm/assets/14147217/4cffb737-c60d-413f-b23a-32abcd06ce8f height=64 alt=yuno-shooting />
<img src=https://github.com/Atomofiron/img2fbm/assets/14147217/8c9b2f13-5d29-4edb-ac07-9ad4f23a9a5c height=64 alt=yuno-shooting_preview />

