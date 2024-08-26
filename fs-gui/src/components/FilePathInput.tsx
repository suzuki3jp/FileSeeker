import type { FC, Dispatch, SetStateAction, ChangeEvent } from "react";
import { Grid, TextField, Button } from "@mui/material";
import { open } from "@tauri-apps/api/dialog";

export const FilePathInput: FC<FilePathInputProps> = (
  props: FilePathInputProps
) => {
  const { filePath, setFilePath } = props;

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
    <Grid item xs={8}>
      <Grid container spacing={2} alignItems="center">
        <Grid item xs={10}>
          <TextField
            size="small"
            fullWidth
            value={filePath}
            onChange={textFieldChangeHandler}
          ></TextField>
        </Grid>
        <Grid item xs={2}>
          <Button variant="contained" onClick={openFilePickerHandler}>
            Browse
          </Button>
        </Grid>
      </Grid>
    </Grid>
  );
};

export interface FilePathInputProps {
  filePath: string;
  setFilePath: Dispatch<SetStateAction<string>>;
}
