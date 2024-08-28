import type { ChangeEvent, Dispatch, SetStateAction } from "react";
import { useState } from "react";
import { Button, Grid, TextField } from "@mui/material";
import { invoke } from "@tauri-apps/api";
import { enqueueSnackbar } from "notistack";
import { open } from "@tauri-apps/api/dialog";

import { InvokeAnalyzeResult } from "../typings";

export const ActionBar = (props: ActionBarProps) => {
  const [filePath, setFilePath] = useState("");
  const { setResults } = props;

  const startButtonHandler = async () => {
    const path = filePath;
    try {
      const result = await invoke<InvokeAnalyzeResult>("analyze", { path });
      setResults(result);
    } catch (error) {
      enqueueSnackbar({ message: String(error), variant: "error" });
    }
  };

  const openFilePickerHandler = async () => {
    const result = await open({ directory: true, multiple: false });
    if (typeof result !== "string") return;
    setFilePath(result);
  };

  // テキストフィールドを手入力でも変更できるために必要
  const textFieldChangeHandler = (e: ChangeEvent<HTMLInputElement>) => {
    setFilePath(e.target.value);
  };

  return (
    <>
      <Grid item xs={12}>
        <Grid container spacing={2} alignItems="center" xs={12}>
          <Grid item xs={8}>
            <TextField
              size="small"
              fullWidth
              value={filePath}
              onChange={textFieldChangeHandler}
            ></TextField>
          </Grid>
          <Grid item>
            <Button variant="contained" onClick={openFilePickerHandler}>
              Browse
            </Button>
          </Grid>
          <Grid item>
            <Button
              variant="contained"
              color="warning"
              onClick={startButtonHandler}
            >
              Start
            </Button>
          </Grid>
        </Grid>
      </Grid>
    </>
  );
};

interface ActionBarProps {
  results: InvokeAnalyzeResult;
  setResults: Dispatch<SetStateAction<InvokeAnalyzeResult>>;
}
