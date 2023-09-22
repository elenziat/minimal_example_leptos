## Expected behaviour

- When the application is started, a resource is constructed that fetches data (here: it waits for 2 seconds)
- During this time the browser should just show "wait for me ..."
- When the data is loaded, the `Child` component should be rendered with "here i am", but this text should be immediately overwritten by the effect with "and here is the data" as the effect should receive new data over a signal.
    - Actually, this would be nice, but I wouldn't consider it a "hard" expectation.
    This is because the conditions to change the text also includes that the `div` element is already mounted.
    (For this example one could change the text even before the `div` is mounted, but for the actual application the `div` is the "home" of a plotly graph, and plotly throws errors if the `div` is unmounted.)

- As the resource is fetched only once, I wouldn't expect a difference between using "Transition" and "Suspense"

- The children of Transition/Suspense are programmed in two variations:
    1. The query of the resource and the Child component are siblings
    2. They are integrated in one single child of Transition/Suspense

- There are some debug outputs to the console to see which functions and effects are executed and to see the state of the `div`-element of the Child component.

## Leptos 0.5.0-rc2

- Transition + 1.: Works like expected. The Child is constructed twice: One time directly after the start and a second time when the data is available. The first time the div element is unmounted, the second time it is mounted. The displayed text ends up to be "and here is the data", as expected.

- (?) Transition + 2.: No Child is constructed before the resource has fetched the data (as expected). But then the Child is constructed twice! (The first time with unmounted div and the second time with mounted div.) The displayed text ends up to be "and here is the data", as expected.

- (?) Suspense + 1.: The Child is constructed once at start. After the fallback display of "wait for me..." the text changes to "here i am". When the effect is run again with the received data, the div is still unmounted and therefore the displayed text ends up being "here i am" instead of "and here is the data".

- (?) Suspense + 2.: The Child is constructed once after the data is fetched (as expected). But the div is still unmounted and the effect doesn't change the text to "and here is the data"

The overall behavior in the last two cases is not ideal for my application but still as expected. But why is it different when Transition is used instead of Suspense?


## Leptos 0.4.10

The code should be identical to the 0.5.0-rc2 version, only with the addition of `cx` where needed.

A basic difference between 0.4.10 and 0.5.0-rc2 is that in 0.4.10 the effect of the Child is called twice the first time, once with the node reference being None and once with it being Some.

- (!) Transition + 1.: The Child is (comparing to 0.5.0-rc2 somewhat unexpected) constructed only once, at startup. The browser shows directly "here i am" instead of the fallback text "wait for me...", even if the log confirms that the data is not fetched yet. When the data is fetched, the text changes to "and here is the data".

- (!) Transition + 2.: No text is shown at all while the resource loads the data. After the data is loaded the Child is constructed once. As also in the second invocation of the effect the div is still unmounted the displayed texts ends up being "here i am"

- Suspense + 1.: The Child is constructed once at startup.  The fallback text is displayed as expected. After the data is loaded the effect of the Child runs again, but as the div is still unmounted the text is "here i am" at the end.

- Suspense + 2.: The Child is constructed once after the data is fetched (as expected). During this time the fallback text is shown. The data is received by the effect, but the div is still unmounted, that is, we end up seeing "here i am".
