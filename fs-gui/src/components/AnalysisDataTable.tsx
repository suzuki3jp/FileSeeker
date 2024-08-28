import type { Dispatch, SetStateAction } from "react";
import {
  TableContainer,
  Table,
  TableHead,
  TableBody,
  Paper,
  TableRow,
  TableCell,
  Grid,
} from "@mui/material";

import { InvokeAnalyzeResult } from "../typings";

export const AnalysisDataTable = (props: AnalysisDataTableProps) => {
  const { results } = props;

  return (
    <Grid item xs={12}>
      <TableContainer component={Paper} sx={{ marginTop: "2%" }}>
        <Table size="small" sx={{ tableLayout: "fixed" }}>
          <TableHead>
            <TableRow>
              <TableCell width={"60%"}>Path</TableCell>
              <TableCell width={"10%"}>Extension</TableCell>
              <TableCell width={"10%"}>Charactors</TableCell>
              <TableCell width={"10%"}>Lines</TableCell>
              <TableCell width={"10%"}>Size (B)</TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {results.map((data) => {
              const displayData: DisplayAnalysisData = {
                path: data.path_parts.join("/"),
                extension: data.extension,
                char: data.char,
                line: data.line,
                size: data.size,
              };
              const { path, extension, char, line, size } = displayData;

              return (
                <TableRow key={path}>
                  <TableCell component={"th"} scope="cell" width={"60%"}>
                    {path}
                  </TableCell>
                  <TableCell width={"10%"}>{extension}</TableCell>
                  <TableCell width={"10%"}>{char}</TableCell>
                  <TableCell width={"10%"}>{line}</TableCell>
                  <TableCell width={"10%"}>{size}</TableCell>
                </TableRow>
              );
            })}
          </TableBody>
        </Table>
      </TableContainer>
    </Grid>
  );
};

export interface AnalysisDataTableProps {
  results: InvokeAnalyzeResult;
  setResults: Dispatch<SetStateAction<InvokeAnalyzeResult>>;
}

interface DisplayAnalysisData {
  path: string;
  extension: string;
  char: number;
  line: number;
  size: number;
}
