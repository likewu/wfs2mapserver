/*
 * SPDX-FileCopyrightText: 2023 Espressif Systems (Shanghai) CO LTD
 *
 * SPDX-License-Identifier: CC0-1.0
 */
 
// This demo UI is adapted from LVGL official example: https://docs.lvgl.io/master/examples.html#scatter-chart

#include <lvgl.h>
#include <stdio.h>
 
static void draw_event_cb(lv_event_t *e)
{
  /*lv_draw_line_dsc_t line_dsc;
  lv_draw_line_dsc_init(&line_dsc);
  lv_obj_init_draw_line_dsc(obj, LV_PART_INDICATOR, &line_dsc); // Fetching style for LV_PART_INDICATOR
  line_dsc.p1 = point_a;
  line_dsc.p2 = point_b;
  lv_draw_line(layer, &line_dsc);*/

/*    lv_obj_draw_part_dsc_t *dsc = lv_event_get_draw_part_dsc(e);
    if (dsc->part == LV_PART_ITEMS) {
        lv_obj_t *obj = lv_event_get_target(e);
        lv_chart_series_t *ser = lv_chart_get_series_next(obj, NULL);
        uint32_t cnt = lv_chart_get_point_count(obj);
        dsc->rect_dsc->bg_opa = (LV_OPA_COVER *  dsc->id) / (cnt - 1);
 
        lv_coord_t *x_array = lv_chart_get_x_array(obj, ser);
        lv_coord_t *y_array = lv_chart_get_y_array(obj, ser);
        uint32_t start_point = lv_chart_get_x_start_point(obj, ser);
        uint32_t p_act = (start_point + dsc->id) % cnt;
        lv_opa_t x_opa = (x_array[p_act] * LV_OPA_50) / 200;
        lv_opa_t y_opa = (y_array[p_act] * LV_OPA_50) / 1000;
 
        dsc->rect_dsc->bg_color = lv_color_mix(lv_palette_main(LV_PALETTE_RED),
                                               lv_palette_main(LV_PALETTE_BLUE),
                                               x_opa + y_opa);
    }*/
}
 
static void add_data(lv_timer_t *timer)
{
    lv_obj_t *chart = lv_timer_get_user_data(timer);
    lv_chart_set_next_value2(chart, lv_chart_get_series_next(chart, NULL), lv_rand(0, 200), lv_rand(0, 1000));
}
 
void example_lvgl_demo_ui(/*lv_disp_t *disp*/)
{
    //lv_obj_t *scr = lv_disp_get_scr_act(disp);
    lv_obj_t *scr = lv_screen_active();
    lv_obj_t *chart = lv_chart_create(scr);
    lv_obj_set_size(chart, 200, 150);
    lv_obj_align(chart, LV_ALIGN_CENTER, 0, 0);
    lv_obj_add_event_cb(chart, draw_event_cb, LV_EVENT_DRAW_MAIN_BEGIN, NULL);
    lv_obj_set_style_line_width(chart, 0, LV_PART_ITEMS);   /*Remove the lines*/
 
    lv_chart_set_type(chart, LV_CHART_TYPE_SCATTER);
 
    //lv_chart_set_axis_tick(chart, LV_CHART_AXIS_PRIMARY_X, 5, 5, 5, 1, true, 30);
    //lv_chart_set_axis_tick(chart, LV_CHART_AXIS_PRIMARY_Y, 10, 5, 6, 5, true, 50);
    //lv_scale_set_total_tick_count(chart, 5);
    //lv_scale_set_major_tick_every(chart, 1);

    lv_chart_set_range(chart, LV_CHART_AXIS_PRIMARY_X, 0, 200);
    lv_chart_set_range(chart, LV_CHART_AXIS_PRIMARY_Y, 0, 1000);
 
    lv_chart_set_point_count(chart, 50);
 
    lv_chart_series_t *ser = lv_chart_add_series(chart, lv_palette_main(LV_PALETTE_RED), LV_CHART_AXIS_PRIMARY_Y);
    for (int i = 0; i < 50; i++) {
        lv_chart_set_next_value2(chart, ser, lv_rand(0, 200), lv_rand(0, 1000));
    }
 
    lv_timer_create(add_data, 100, chart);
}