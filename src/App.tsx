import { createResource, createSignal} from 'solid-js';
import { invoke } from '@tauri-apps/api/tauri';
import './App.css';
interface ButtonProps {
    label: string;
    onClick: () => void;
}

const Button = (props: ButtonProps) => {
    return (
        <button
            onClick={props.onClick}
            class="col-span-1 p-4 border rounded-md text-white bg-gray-600 hover:bg-gray-500 focus:outline-none"
        >
            {props.label}
        </button>
    );
};
function App() {
    const [inputValue, setInputValue] = createSignal('');

    const buttonLabels = [
        '7',
        '8',
        '9',
        '/',
        '4',
        '5',
        '6',
        '*',
        '1',
        '2',
        '3',
        '-',
        '0',
        '.',
        '+',
        '=',
        '√',
        'cos',
        '(',
        ')', 
    ];

    async function calculate(): Promise<string> {
        return invoke('calculate', { input: inputValue() });
    }

    const handleButtonClick = async (label: string) => {
        if (label === '=') {
            setInputValue(await calculate());
        } else {
            setInputValue((prevValue) => prevValue + label);
        }
    };

    const [result] = createResource(inputValue, calculate);
    return (
        <div class="bg-gray-900 h-screen flex items-center justify-center w-screen">
            <div class="bg-gray-800 rounded-lg shadow-md p-6 w-screen h-screen">
                <div class="relative">
                    <input
                        type="text"
                        class="w-full h-16 p-2 pt-1 border bg-gray-700 rounded-md text-white mb-2"
                        value={inputValue()}
			onInput={(e)=>setInputValue(e.target.value)}
                    />
                    <div class="absolute bottom-2 right-2 text-gray-500">
                        {result()}
                    </div>
                </div>
                <div class="grid grid-cols-4 gap-4 mt-4">
                    {buttonLabels.map((label) => (
                        <Button
                            label={label}
                            onClick={() => handleButtonClick(label)}
                        />
                    ))}
                </div>
            </div>
        </div>
    );
}

export default App;
