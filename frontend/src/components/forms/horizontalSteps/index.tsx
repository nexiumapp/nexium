import { h, FunctionalComponent } from "preact";

import * as style from "./style.scss";

/**
 * Render a horizontal step indicator.
 * @param props The props to render with.
 * @returns The JSX for the bar.
 */
export const HorizontalSteps: FunctionalComponent<Props> = (props) => {
    const clickListener = (item: number) => {
        if (!props.onClick) return;
        if (item >= props.activeStep) return;

        props.onClick(item);
    };

    let items = [];
    const itemWidth = `${(1 / props.stepCount) * 100}%`;

    for (let i = 1; i <= props.stepCount; i++) {
        const active = props.activeStep === i ? style.active : "";
        const clickable =
            props.onClick && i < props.activeStep ? style.clickable : "";
        const classes = `${active} ${clickable}`;

        const item = (
            <li
                class={classes}
                style={{ width: itemWidth }}
                key={i}
                onClick={() => clickListener(i)}
            />
        );
        items.push(item);
    }

    return <ul class={style.bar}>{items}</ul>;
};

interface Props {
    // Amount of steps in the bar.
    stepCount: number;
    // Current active step.
    // This is one indexed, so if there are 3 steps, the last active step is `3`.
    activeStep: number;

    onClick?: (item: number) => void;
}
