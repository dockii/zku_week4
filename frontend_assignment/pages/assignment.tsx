import * as React from 'react';

import Box from '@mui/material/Box';
import Button from '@mui/material/Button';
import TextField from '@mui/material/TextField';
import { Stack } from '@mui/material'
import { useForm } from "react-hook-form";
import { yupResolver } from '@hookform/resolvers/yup';
import * as yup from "yup";

import Greeter from "artifacts/contracts/Greeters.sol/Greeters.json";
import { Contract, providers, utils } from "ethers";

// form inputs
type Inputs = {
    name: string,
    age: number,
    address: string,
};

// for inputs schema validation
const schema = yup.object({
    name: yup.string().required(),
    age: yup.number().positive().integer().required(),
    address: yup.string().required(),
}).required();


export default function Assignment() {

    // initialize state for events
    const [events, setEvents] = React.useState<string[]>([]);

    // initialize for state and setup yup validation for the fields
    const { handleSubmit, register, formState: { errors } } = useForm<Inputs>({
        resolver: yupResolver(schema)
    });

    // log form data on submit
    const onSubmit = (data: Inputs) => console.log(data);


    // setup NewGreeting event listeners on mount
    React.useEffect(() => {
        // initialize rpc provider and initialize greeter contract
        const provider = new providers.JsonRpcProvider("http://localhost:8545")
        const contract = new Contract("0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512", Greeter.abi, provider);

        // add event listener
        contract.on("NewGreeting", (greetingBytes) => {
            const event = utils.parseBytes32String(greetingBytes);
            setEvents((currEvents) => [...currEvents, event]); // extend the state with new event
        });

        // remove listener on unmount
        return () => contract.removeAllListeners();
    }, []);

    return (
        <Box sx={{
            maxWidth: 300,
            margin: '30px auto 0 auto',
            padding: '20px',
            background: '#e3f2fd'
        }}>
            <form onSubmit={handleSubmit(onSubmit)}>
                <Stack direction={'column'} spacing='20px'>
                    <TextField
                        required
                        id="name"
                        {...register("name")} // register the input with form state
                        label="Name"
                        variant="outlined"
                        error={errors.name !== undefined} // match error property to error state
                        helperText={errors.name?.message} // error text
                    />
                    <TextField
                        required
                        id="age"
                        error={errors.age !== undefined}
                        {...register("age")}
                        label="Age"
                        type="number"
                        variant="outlined"
                        InputLabelProps={{
                            shrink: true,
                        }}
                        helperText={errors.age?.message}
                    />
                    <TextField
                        required
                        id="address"
                        {...register("address")}
                        label="Address"
                        variant="outlined"
                        error={errors.address !== undefined}
                        helperText={errors.address?.message}
                    />
                    <Button type="submit" variant="contained">Submit</Button>
                    <hr />
                    <TextField id="greetings" label="events" margin="normal" multiline disabled value={events?.join("\n")} /> {/* disabled, only for events display */}
                </Stack>
            </form>
        </Box>
    );
}