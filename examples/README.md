# Examples
__Iced moves fast and the `master` branch can contain breaking changes!__ If
you want to learn about a specific release, check out [the release list].

[the release list]: https://github.com/hecrj/iced/releases

## [Tour](tour)
A simple UI tour that can run both on native platforms and the web! It showcases different widgets that can be built using Iced.

The __[`main`](tour/src/main.rs)__ file contains all the code of the example! All the cross-platform GUI is defined in terms of __state__, __messages__, __update logic__ and __view logic__.

<div align="center">
  <a href="https://gfycat.com/politeadorableiberianmole">
    <img src="https://thumbs.gfycat.com/PoliteAdorableIberianmole-small.gif">
  </a>
</div>

[`iced_winit`]: ../winit
[`iced_native`]: ../native
[`iced_wgpu`]: ../wgpu
[`iced_web`]: ../web
[`winit`]: https://github.com/rust-windowing/winit
[`wgpu`]: https://github.com/gfx-rs/wgpu-rs

You can run the native version with `cargo run`:
```
cargo run --package tour
```

The web version can be run by following [the usage instructions of `iced_web`] or by accessing [iced.rs](https://iced.rs/)!

[the usage instructions of `iced_web`]: ../web#usage

## [Todos](todos)
A todos tracker inspired by [TodoMVC]. It showcases dynamic layout, text input, checkboxes, scrollables, icons, and async actions! It automatically saves your tasks in the background, even if you did not finish typing them.

The example code is located in the __[`main`](todos/src/main.rs)__ file.

<div align="center">
  <a href="https://gfycat.com/littlesanehalicore">
    <img src="https://thumbs.gfycat.com/LittleSaneHalicore-small.gif" height="400px">
  </a>
</div>

You can run the native version with `cargo run`:
```
cargo run --package todos
```
We have not yet implemented a `LocalStorage` version of the auto-save feature. Therefore, it does not work on web _yet_!

[TodoMVC]: http://todomvc.com/

## [Styling](styling)
An example showcasing custom styling with a light and dark theme.

The example code is located in the __[`main`](styling/src/main.rs)__ file.

<div align="center">
  <a href="https://user-images.githubusercontent.com/518289/71867993-acff4300-310c-11ea-85a3-d01d8f884346.gif">
    <img src="https://user-images.githubusercontent.com/518289/71867993-acff4300-310c-11ea-85a3-d01d8f884346.gif" height="400px">
  </a>
</div>

You can run it with `cargo run`:
```
cargo run --package styling
```

## Extras
A bunch of simpler examples exist:

- [`bezier_tool`](bezier_tool), a Paint-like tool for drawing Bezier curves using [`lyon`].
- [`counter`](counter), the classic counter example explained in the [`README`](../README.md).
- [`custom_widget`](custom_widget), a demonstration of how to build a custom widget that draws a circle.
- [`events`](events), a log of native events displayed using a conditional `Subscription`.
- [`geometry`](geometry), a custom widget showcasing how to draw geometry with the `Mesh2D` primitive in [`iced_wgpu`](../wgpu).
- [`pokedex`](pokedex), an application that displays a random Pokédex entry (sprite included!) by using the [PokéAPI].
- [`progress_bar`](progress_bar), a simple progress bar that can be filled by using a slider.
- [`stopwatch`](stopwatch), a watch with start/stop and reset buttons showcasing how to listen to time.
- [`svg`](svg), an application that renders the [Ghostscript Tiger] by leveraging the `Svg` widget.

All of them are packaged in their own crate and, therefore, can be run using `cargo`:
```
cargo run --package <example>
```

[`lyon`]: https://github.com/nical/lyon
[PokéAPI]: https://pokeapi.co/
[Ghostscript Tiger]: https://commons.wikimedia.org/wiki/File:Ghostscript_Tiger.svg

## [Coffee]
Since [Iced was born in May], it has been powering the user interfaces in
[Coffee], an experimental 2D game engine.


<div align="center">
  <a href="https://gfycat.com/gloomyweakhammerheadshark">
    <img src="https://thumbs.gfycat.com/GloomyWeakHammerheadshark-small.gif">
  </a>
</div>

[Iced was born in May]: https://github.com/hecrj/coffee/pull/35
[`ui` module]: https://docs.rs/coffee/0.3.2/coffee/ui/index.html
[Coffee]: https://github.com/hecrj/coffee