/*
 * Copyright (c) 2018 Jan Van Winkel <jan.van_winkel@dxplore.eu>
 *
 * SPDX-License-Identifier: Apache-2.0
 */
 
#include <zephyr/device.h>
#include <zephyr/devicetree.h>
#include <zephyr/drivers/display.h>
#include <zephyr/drivers/gpio.h>
#include <lvgl.h>
#include <stdio.h>
#include <string.h>
#include <zephyr/kernel.h>
#include <lvgl_input_device.h>
 
#define LOG_LEVEL CONFIG_LOG_DEFAULT_LEVEL
#include <zephyr/logging/log.h>
LOG_MODULE_REGISTER(app);
 
#define LED0_NODE DT_ALIAS(led0)
static const struct gpio_dt_spec led = GPIO_DT_SPEC_GET(LED0_NODE, gpios);
 
static uint32_t count;
 
#ifdef CONFIG_GPIO
static struct gpio_dt_spec button_gpio = GPIO_DT_SPEC_GET_OR(DT_ALIAS(sw0), gpios, {0});
static struct gpio_callback button_callback;
 
static void button_isr_callback(const struct device *port, struct gpio_callback *cb, uint32_t pins)
{
  ARG_UNUSED(port);
  ARG_UNUSED(cb);
  ARG_UNUSED(pins);
 
  count = 0;
}
#endif /* CONFIG_GPIO */
 
#ifdef CONFIG_LV_Z_ENCODER_INPUT
static const struct device *lvgl_encoder =
  DEVICE_DT_GET(DT_COMPAT_GET_ANY_STATUS_OKAY(zephyr_lvgl_encoder_input));
#endif /* CONFIG_LV_Z_ENCODER_INPUT */
 
#ifdef CONFIG_LV_Z_KEYPAD_INPUT
static const struct device *lvgl_keypad =
  DEVICE_DT_GET(DT_COMPAT_GET_ANY_STATUS_OKAY(zephyr_lvgl_keypad_input));
#endif /* CONFIG_LV_Z_KEYPAD_INPUT */
 
static void lv_btn_click_callback(lv_event_t *e)
{
  ARG_UNUSED(e);
 
  count = 0;
}
 
int main(void)
{
  char count_str[11] = {0};
  const struct device *display_dev;
  lv_obj_t *hello_world_label;
  lv_obj_t *count_label;
 
  display_dev = DEVICE_DT_GET(DT_CHOSEN(zephyr_display));
  if (!device_is_ready(display_dev)) {
    LOG_ERR("Device not ready, aborting test");
    return 0;
  }
 
  lv_obj_t *hello_world_button;
 
  hello_world_button = lv_btn_create(lv_scr_act());
  lv_obj_align(hello_world_button, LV_ALIGN_CENTER, 0, -15);
  lv_obj_add_event_cb(hello_world_button, lv_btn_click_callback, LV_EVENT_CLICKED, NULL);
  hello_world_label = lv_label_create(hello_world_button);
 
  lv_label_set_text(hello_world_label, "Hello world!");
  lv_obj_align(hello_world_label, LV_ALIGN_CENTER, 0, 0);
 
  count_label = lv_label_create(lv_scr_act());
  lv_obj_align(count_label, LV_ALIGN_BOTTOM_MID, 0, 0);
 
  lv_task_handler();
  display_blanking_off(display_dev);
 
  gpio_pin_configure_dt(&led, GPIO_OUTPUT);
  gpio_pin_set(led.port, led.pin, 1);
  display_blanking_off(display_dev);
 
  while (1) {
    if ((count % 100) == 0U) {
      sprintf(count_str, "%d", count / 100U);
      lv_label_set_text(count_label, count_str);
    }
    lv_task_handler();
    ++count;
    k_sleep(K_MSEC(10));
  }
}