import { h, FunctionalComponent } from "preact";
import { useState } from "preact/hooks";

import { HorizontalSteps } from "/src/components/forms";

import { NameStep } from "./nameStep";
import { AuthStep } from "./authStep";

/**
 * Register screen.
 * Includes multiple steps, which are not routed.
 * @returns The JSX of the registration screen.
 */
export const Register: FunctionalComponent = () => {
    const [step, setStep] = useState(1);
    const [name, setName] = useState("");
    const [username, setUsername] = useState("");

    const changeStep = (newStep: number) => {
        setStep(newStep);
    };

    let stepComponent = <div>asdasd</div>;

    if (step === 1) {
        stepComponent = (
            <NameStep
                name={name}
                username={username}
                setName={setName}
                setUsername={setUsername}
                step={step}
                changeStep={changeStep}
            />
        );
    } else if (step === 2) {
        stepComponent = <AuthStep step={step} changeStep={changeStep} />;
    }

    return (
        <main>
            <h2>Welcome! Let's Get Started!</h2>
            <h4>
                Or{" "}
                <a href="/auth/login" alt="Login">
                    login
                </a>{" "}
                instead.
            </h4>
            <HorizontalSteps
                stepCount={2}
                activeStep={step}
                onClick={(i) => changeStep(i)}
            />
            {stepComponent}
        </main>
    );
};
