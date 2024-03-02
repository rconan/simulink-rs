/*
 * File: M1SA_Control_OA.h
 *
 * Code generated for Simulink model 'M1SA_Control_OA'.
 *
 * Model version                  : 9.6
 * Simulink Coder version         : 9.8 (R2022b) 13-May-2022
 * C/C++ source code generated on : Fri Apr 14 13:59:52 2023
 *
 * Target selection: ert.tlc
 * Embedded hardware selection: Intel->x86-64 (Linux 64)
 * Code generation objective: Execution efficiency
 * Validation result: Not run
 */

#ifndef RTW_HEADER_M1SA_Control_OA_h_
#define RTW_HEADER_M1SA_Control_OA_h_
#ifndef M1SA_Control_OA_COMMON_INCLUDES_
#define M1SA_Control_OA_COMMON_INCLUDES_
#include "rtwtypes.h"
#endif                                 /* M1SA_Control_OA_COMMON_INCLUDES_ */

#include "M1SA_Control_OA_types.h"
#include <string.h>
#include "rt_defines.h"

/* Block signals and states (default storage) for system '<Root>' */
typedef struct {
  real_T Fxcontroller_DSTATE[3];       /* '<S2>/Fx controller' */
  real_T Fycontroller_DSTATE[3];       /* '<S2>/Fy controller' */
  real_T Fzcontroller_DSTATE[3];       /* '<S2>/Fz controller' */
  real_T Mxcontroller_DSTATE[3];       /* '<S2>/Mx controller' */
  real_T Mycontroller_DSTATE[3];       /* '<S2>/My controller' */
  real_T Mzcontroller_DSTATE[3];       /* '<S2>/Mz controller' */
  real_T OA_SA_dyn_states[335];        /* '<S1>/OA_SA_dyn' */
} DW_M1SA_Control_OA_T;

/* Constant parameters (default storage) */
typedef struct {
  /* Expression: m1sys{1}.Kbal
   * Referenced by: '<S1>/OAseg_Kbal'
   */
  real_T OAseg_Kbal_Gain[2010];
} ConstP_M1SA_Control_OA_T;

/* External inputs (root inport signals with default storage) */
typedef struct {
  real_T LC_FxyzMxyz_CG[6];            /* '<Root>/LC_FxyzMxyz_CG' */
  real_T SA_offsetF_cmd[335];          /* '<Root>/SA_offsetF_cmd' */
} ExtU_M1SA_Control_OA_T;

/* External outputs (root outports fed by signals with default storage) */
typedef struct {
  real_T Res_Act_F[335];               /* '<Root>/Res_Act_F' */
} ExtY_M1SA_Control_OA_T;

/* Real-time Model Data Structure */
struct tag_RTM_M1SA_Control_OA_T {
  DW_M1SA_Control_OA_T *dwork;
};

/* Constant parameters (default storage) */
extern const ConstP_M1SA_Control_OA_T M1SA_Control_OA_ConstP;

/* Model entry point functions */
extern void M1SA_Control_OA_initialize(RT_MODEL_M1SA_Control_OA_T *const
  M1SA_Control_OA_M);
extern void M1SA_Control_OA_step(RT_MODEL_M1SA_Control_OA_T *const
  M1SA_Control_OA_M, ExtU_M1SA_Control_OA_T *M1SA_Control_OA_U,
  ExtY_M1SA_Control_OA_T *M1SA_Control_OA_Y);
extern void M1SA_Control_OA_terminate(RT_MODEL_M1SA_Control_OA_T *const
  M1SA_Control_OA_M);

/*-
 * The generated code includes comments that allow you to trace directly
 * back to the appropriate location in the model.  The basic format
 * is <system>/block_name, where system is the system number (uniquely
 * assigned by Simulink) and block_name is the name of the block.
 *
 * Note that this particular code originates from a subsystem build,
 * and has its own system numbers different from the parent model.
 * Refer to the system hierarchy for this subsystem below, and use the
 * MATLAB hilite_system command to trace the generated code back
 * to the parent model.  For example,
 *
 * hilite_system('M1DCS_2_rust/M1SA_Control_OA')    - opens subsystem M1DCS_2_rust/M1SA_Control_OA
 * hilite_system('M1DCS_2_rust/M1SA_Control_OA/Kp') - opens and selects block Kp
 *
 * Here is the system hierarchy for this model
 *
 * '<Root>' : 'M1DCS_2_rust'
 * '<S1>'   : 'M1DCS_2_rust/M1SA_Control_OA'
 * '<S2>'   : 'M1DCS_2_rust/M1SA_Control_OA/OA_ofl_SSdtC'
 */
#endif                                 /* RTW_HEADER_M1SA_Control_OA_h_ */

/*
 * File trailer for generated code.
 *
 * [EOF]
 */
